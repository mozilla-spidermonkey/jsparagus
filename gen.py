#!/usr/bin/env python3

"""gen.py - Fifth stab at a parser generator.

**Nature of a grammar.**
A grammar is a dictionary {str: [[symbol]]} mapping names of nonterminals to lists of productions.
A production is a nonempty list of symbols.
Each symbol specifies either a kind of terminal or a nonterminal (by name).

**Context of the generated parser.**
The user passes to each method an object representing the input sequence.
This object must support two methods:

*   `src.peek()` returns the kind of the next token, or `None` at the end of input.

*   `src.take(kind)` throws an exception if `src.peek() != kind`;
    otherwise, it removes the next token from the input stream and returns it.
    The special case `src.take(None)` checks that the input stream is empty:
    if so, it returns None; if not, it throws.

**Simplifying assumptions about the grammar.**
No productions may be empty. Empty productions would complicate table generation.

We assume that every nonterminal matches at least one string of finite length.
It's not a bug if it doesn't, but it would be nice to check.
"""

import collections
import pprint
import textwrap
import io
from pgen_runtime import ERROR, ACCEPT


# *** What is a grammar? ******************************************************
#
# A grammar is a dictionary mapping nonterminal names to lists of right-hand
# sides. Each right-hand side (also called a "production") is a list whose
# elements all pass exactly one of the predicates below.
#
# The most common elements are terminals and nonterminals, so a grammar usually
# looks a lot like this:
def example_grammar():
    return {
        'expr': [
            ['term'],
            ['expr', '+', 'term'],
            ['expr', '-', 'term'],
        ],
        'term': [
            ['unary'],
            ['term', '*', 'unary'],
            ['term', '/', 'unary'],
        ],
        'unary': [
            ['prim'],
            ['-', 'unary'],
        ],
        'prim': [
            ['NUM'],
            ['VAR'],
            ['(', 'expr', ')'],
        ],
    }

def is_nt(grammar, element):
    return isinstance(element, str) and element in grammar


def is_terminal(grammar, element):
    return isinstance(element, str) and not is_nt(grammar, element)


Optional = collections.namedtuple("Optional", "inner")
Optional.__doc__ = """Optional(nt) matches either nothing or the given nt."""

def is_optional(element):
    return isinstance(element, Optional)


LookaheadRule = collections.namedtuple("LookaheadRule", "set positive")
LookaheadRule.__doc__ = """\
LookaheadRule(set, pos) imposes a lookahead restriction on whatever follows.

It never consumes any tokens itself. Instead, the right-hand side
[LookaheadRule(frozenset(['a', 'b']), False), 'Thing']
matches a Thing that does not start with the token `a` or `b`.
"""


def is_lookahead_rule(element):
    return isinstance(element, LookaheadRule)


def lookahead_contains(rule, t):
    return (rule is None
            or (t in rule.set if rule.positive else t not in rule.set))


def lookahead_intersect(a, b):
    if a is None:
        return b
    elif b is None:
        return a
    elif a.positive:
        if b.positive:
            return LookaheadRule(a.set & b.set, True)
        else:
            return LookaheadRule(a.set - b.set, True)
    else:
        if b.positive:
            return LookaheadRule(b.set - a.set, True)
        else:
            return LookaheadRule(a.set | b.set, False)


def fix(f, start):
    """Compute the least fix point of `f`, the hard way, starting from `start`."""
    prev, current = start, f(start)
    while current != prev:
        prev, current = current, f(current)
    return current


def empty_nt_set(grammar):
    """Return the set of all nonterminals in `grammar` that can produce the empty string."""
    def step(empties):
        def rhs_is_empty(nt, rhs):
            return all(is_lookahead_rule(e)
                       or is_optional(e)
                       or (is_nt(grammar, e) and e in empties)
                       for e in rhs)
        return set(nt
                   for nt, rhs_list in grammar.items()
                   if any(rhs_is_empty(nt, rhs) for rhs in rhs_list))

    return fix(step, set())


def check_cycle_free(grammar):
    """Throw an exception if any nonterminal in `grammar` produces itself
    via a cycle of 1 or more productions.
    """
    empties = empty_nt_set(grammar)

    # OK, first find out which nonterminals directly produce which other
    # nonterminals (after possibly erasing some optional/empty nts).
    direct_produces = {}
    for orig in grammar:
        direct_produces[orig] = set()
        for source_rhs in grammar[orig]:
            for rhs, _r in expand_optional_symbols(source_rhs):
                result = []
                all_possibly_empty_so_far = True
                # If we break out of the following loop, that means it turns
                # out that this production does not produce *any* strings that
                # are just a single nonterminal.
                for e in rhs:
                    if is_terminal(grammar, e):
                        break  # no good, this production contains a terminal
                    elif is_nt(grammar, e):
                        if e in empties:
                            result.append(e)
                        else:
                            if not all_possibly_empty_so_far:
                                break # no good, we have 2+ nonterminals that can't be empty
                            all_possibly_empty_so_far = False
                            result = [e]
                    elif is_optional(e):
                        if is_nt(grammar, e.inner):
                            result.append(e.inner)
                    else:
                        assert is_lookahead_rule(e)
                        pass # ignore the restriction - we lose a little precision here
                else:
                    # If we get here, we didn't break, so our results are good!
                    # nt can definitely produce all the nonterminals in result.
                    direct_produces[orig] |= set(result)

    def step(produces):
        return {
            orig: dest | set(b for a in dest for b in produces[a])
            for orig, dest in produces.items()
        }
    produces = fix(step, direct_produces)

    for nt in grammar:
        if nt in produces[nt]:
            raise ValueError("invalid grammar: nonterminal {} can produce itself".format(nt))


def check(grammar):
    """Enforce a few basic rules about the grammar.

    1.  The grammar is cycle-free.
        A cycle is a set of productions such that any
        nonterminal `q` has `q ==>+ q` (produces itself via at least one step).

    2.  No LookaheadRule appears at the end of a production (or before elements
        that can produce the empty string).

    If the grammar breaks either rule, throw.
    """

    check_cycle_free(grammar)
    for nt in grammar:
        for rhs_with_options in grammar[nt]:
            for rhs, _r in expand_optional_symbols(rhs_with_options):
                if rhs and is_lookahead_rule(rhs[-1]):
                    # XXX this is insufficient now
                    raise ValueError("invalid grammar: lookahead restriction at end of production: " +
                                     production_to_str(grammar, nt, rhs_with_options))


def gensym(grammar, nt):
    """ Come up with a symbol name that's not already being used in the given grammar. """
    assert is_nt(grammar, nt)
    while nt in grammar:
        nt += "_"
    return nt



def clone_grammar(grammar):
    return {nt: [prod[:] for prod in prods] for nt, prods in grammar.items()}


EMPTY = "(empty)"
END = None


def start_sets(grammar):
    """Compute the start sets for terminals and nonterminals in a grammar.

    A symbol's start set is the set of tokens that a match for that symbol
    may start with, plus EMPTY if the symbol can match the empty string.
    """

    # The rules, in pseudocode, are:
    #
    # if is_terminal(t) then start[t] == {t}
    # for nt in grammar, prod in grammar[nt],
    #     if all(for e in prod, epsilon in start[e]), then epsilon in start[nt]
    # for nt in grammar, prod in grammar[nt], 0 <= i < len(prod),
    #     if all(for e in prod[:i], epsilon in start[e]),
    #         for t in start[prod[i]], if is_terminal(t) then t in start[nt]
    #
    # Since this definition is rather circular, we have to iterate to a least
    # fix point. In the actual code, we don't bother computing and storing
    # start sets for terminals.

    start = {nt: set() for nt in grammar}
    done = False
    while not done:
        done = True
        for nt, prods in grammar.items():
            # Compute start set for each `prod` based on `start` so far.
            # Could be incomplete, but we'll ratchet up as we iterate.
            nt_start = set.union(*[seq_start(grammar, start, prod) for prod in prods])
            if nt_start != start[nt]:
                start[nt] = nt_start
                done = False
    return start


def seq_start(grammar, start, seq):
    """Compute the start set for a sequence of elements."""
    s = {EMPTY}
    for i, e in enumerate(seq):
        if EMPTY not in s:  # preceding elements never match the empty string
            break
        s.remove(EMPTY)
        if is_terminal(grammar, e):
            s.add(e)
        elif is_nt(grammar, e):
            s |= start[e]
        else:
            assert is_lookahead_rule(e)
            future = seq_start(grammar, start, seq[i + 1:])
            if e.positive:
                future &= e.set
            else:
                future -= e.set
            return future
    return s


def follow_sets(grammar, goal):
    """Compute all follow sets for nonterminals in a grammar.

    The follow set for a nonterminal `A`, as defined in the book, is "the set
    of terminals that can appear immediately to the right of `A` in some
    sentential form"; plus, "If `A` can be the rightmost symbol in some
    sentential form, then $ is in FOLLOW(A)."

    The `goal` argument is necessary to specify what a sentential form is,
    since sentential forms are partial derivations of a particular goal
    nonterminal.

    Returns a default-dictionary mapping nts to follow sets.
    """

    start = start_sets(grammar)

    # Set of nonterminals already seen, including those we are in the middle of
    # analyzing. The algorithm starts at `goal` and walks all reachable
    # nonterminals, recursively.
    visited = set()

    # The results. By definition, nonterminals that are not reachable from the
    # goal nt have empty follow sets.
    follow = collections.defaultdict(set)

    # If `(x, y) in subsumes_relation`, then x can appear at the end of a
    # production of y, and therefore follow[x] should be <= follow[y].
    # (We could maintain that invariant throughout, but at present we
    # brute-force iterate to a fixed point at the end.)
    subsumes_relation = set()

    # `END` is $. It is, of course, in follow[goal]. It gets into other
    # nonterminals' follow sets through the subsumes relation.
    follow[goal].add(END)

    def visit(nt):
        if nt in visited:
            return
        visited.add(nt)
        for prod in grammar[nt]:
            for i, symbol in enumerate(prod):
                if is_nt(grammar, symbol):
                    visit(symbol)
                    after = seq_start(grammar, start, prod[i + 1:])
                    if EMPTY in after:
                        after.remove(EMPTY)
                        subsumes_relation.add((symbol, nt))
                    follow[symbol] |= after

    visit(goal)

    # Now iterate to a fixed point on the subsumes relation.
    done = False
    while not done:
        done = True # optimistically
        for target, source in subsumes_relation:
            if follow[source] - follow[target]:
                follow[target] |= follow[source]
                done = False

    return follow


def expand_optional_symbols(rhs, start_index=0):
    for i in range(start_index, len(rhs)):
        if is_optional(rhs[i]):
            break
    else:
        yield rhs[start_index:], []
        return

    for expanded, r in expand_optional_symbols(rhs, i + 1):
        # without rhs[i]
        yield rhs[start_index:i] + expanded, [i] + r
        # with rhs[i]
        yield rhs[start_index:i] + [rhs[i].inner] + expanded, r


def make_epsilon_free_step_1(grammar):
    """ Return a clone of `grammar` in which all uses of nonterminals
    that match the empty string are wrapped in Optional.

    `grammar` must already be cycle-free.
    """

    empties = empty_nt_set(grammar)

    def hack(e):
        if is_nt(grammar, e) and e in empties:
            return Optional(e)
        return e

    return {
        nt: [[hack(e) for e in rhs]
             for rhs in rhs_list]
        for nt, rhs_list in grammar.items()
    }


def make_epsilon_free_step_2(grammar, goal):
    """Return a clone of `grammar` with empty right-hand sides removed.

    All empty productions are removed except any for the goal nonterminal,
    so the grammar still recognizes the same language.
    """
    return {
        nt: [rhs for rhs in rhs_list if len(rhs) > 0 or nt == goal]
        for nt, rhs_list in grammar.items()
    }


# *** How to dump stuff *******************************************************

def element_to_str(grammar, e):
    if is_nt(grammar, e):
        return e
    elif is_terminal(grammar, e):
        return '"' + repr(e)[1:-1] + '"'
    elif is_optional(e):
        return element_to_str(grammar, e.inner) + "?"
    elif is_lookahead_rule(e):
        if len(e.set) == 1:
            op = "==" if e.positive else "!="
            s = repr(list(e.set)[0])
        else:
            op = "in" if e.positive else "not in"
            s = repr(set(e.set))
        return "[lookahead {} {}]".format(op, s)
    else:
        return str(e)


def rhs_to_str(grammar, rhs):
    return " ".join(element_to_str(grammar, e) for e in rhs)


def production_to_str(grammar, nt, rhs):
    return "{} ::= {}".format(nt, rhs_to_str(grammar, rhs))


def state_to_str(grammar, prods, state):
    nt, _i, rhs = prods[state.prod_index]
    if state.lookahead is None:
        la = []
    else:
        la = [element_to_str(grammar, state.lookahead)]
    return "{} ::= {}".format(
        nt,
        " ".join([element_to_str(grammar, e) for e in rhs[:state.offset]]
                 + ["\N{MIDDLE DOT}"]
                 + la
                 + [element_to_str(grammar, e) for e in rhs[state.offset:]])
    )


def state_set_to_str(grammar, prods, state_set):
    return "{{{}}}".format(
        ",  ".join(state_to_str(grammar, prods, state) for state in state_set)
    )


def dump_grammar(grammar):
    for nt, prods in grammar.items():
        print(nt + " ::=")
        for rhs in prods:
            if rhs:
                print("   ", rhs_to_str(grammar, rhs))
            else:
                print("   [empty]")
        print()


# *** Parser generation *******************************************************

State = collections.namedtuple("State", "prod_index offset lookahead")

def generate_parser(out, grammar, goal):
    def get_state_set_index(s):
        """ Get a number for a set of states, assigning a new number if needed. """
        successors = frozenset(s)
        if successors in visited_state_sets:
            return visited_state_sets[successors]
        else:
            visited_state_sets[successors] = state_index = len(visited_state_sets)
            ## print("State-set #{} = {}"
            ##       .format(state_index,
            ##               state_set_to_str(grammar, prods, state_set_closure(successors))))
            todo.append(successors)
            return state_index

    def make_state(*args, **kwargs):
        """Create a State tuple and advance it past any lookahead rules.

        The main algorithm assumes that the "next element" in any State is
        never a lookahead rule. We ensure that is true by processing lookahead
        elements before the State is even exposed.

        We don't bother doing extra work here to eliminate lookahead
        restrictions that are redundant with what's coming up next in the
        grammar, like `[lookahead != NUM]` when the production is
        `name ::= IDENT`. We also don't eliminate states that can't match,
        like `name ::= IDENT` when we have `[lookahead not in {IDENT}]`.

        Such silly states can exist; but we would only care if it caused
        get_state_set_index to treat equivalent state-sets as distinct.
        I haven't seen that happen for any grammar yet.
        """
        state = State(*args, **kwargs)
        _nt, _i, rhs = prods[state.prod_index]
        while state.offset < len(rhs) and is_lookahead_rule(rhs[state.offset]):
            state = state._replace(offset=state.offset + 1,
                                   lookahead=lookahead_intersect(state.lookahead, rhs[state.offset]))

        #if state.lookahead is not None:
        if False:  # this block is disabled for now; see comment
            # We want equivalent states to be ==, so the following code
            # canonicalizes lookahead rules, eliminates lookahead rules that
            # are redundant with the upcoming symbols in the rhs, and
            # eliminates states that (due to lookahead rules) won't match
            # anything.
            #
            # This sounds good in theory, and it does reduce the number of
            # States we end up tracking, but I have not found an example where
            # it reduces the number of runtime "parser states" i.e. state-sets.
            # So this code is disabled for now.

            expected = seq_start(grammar, start, rhs[state.offset:])
            if state.lookahead.positive:
                ok_set = expected & state.lookahead.set
            else:
                ok_set = expected - state.lookahead.set

            if len(ok_set) == 0:
                return None  # this state can't match anything
            elif ok_set == expected:
                look = None
            else:
                look = LookaheadRule(frozenset(ok_set), True)
            state = state._replace(lookahead=look)
        return state

    def state_set_closure(state_set):
        """Compute transitive closure of the given state set under left-calls.

        That is, return a superset of state_set that adds every state that's
        reachable from it by "stepping in" to nonterminals without consuming
        any tokens. Note that it's often possible to "step in" repeatedly.

        This is the only part of the system that makes states with lookahead
        restrictions.
        """
        closure = set(state_set)
        closure_todo = list(state_set)
        while closure_todo:
            state = closure_todo.pop(0)
            assert isinstance(state, State)
            _nt, _i, rhs = prods[state.prod_index]
            if state.offset < len(rhs):
                next_symbol = rhs[state.offset]
                if is_nt(grammar, next_symbol):
                    for dest_prod_index, (dest_nt, _i, _rhs) in enumerate(prods):
                        if dest_nt == next_symbol:
                            new_state = make_state(dest_prod_index, 0, state.lookahead)
                            if new_state is not None and new_state not in closure:
                                closure.add(new_state)
                                closure_todo.append(new_state)
        return closure

    def raise_reduce_reduce_conflict(t, i, j):
        nt1, _, rhs1 = prods[i]
        nt2, _, rhs2 = prods[j]

        raise ValueError(
            "reduce-reduce conflict when looking at {!r} "
            "after input matching both:\n"
            "    {}\n"
            "and:\n"
            "    {}\n"
            .format(t,
                    production_to_str(grammar, nt1, rhs1),
                    production_to_str(grammar, nt2, rhs2)))

    def analyze_state_set(current_state_set):
        """Generate the LR parser table entry for a single state set.

        This can be done without iterating. But this function sometimes needs
        state-set-ids for state sets we haven't considered yet, so it calls
        get_state_set_index (a side effect).
        """

        #print("analyzing state set {}".format(state_set_to_str(grammar, current_state_set)))

        # Step 1. Visit every state and list what we want to do for each
        # possible next token.
        shift_states = collections.defaultdict(set)  # maps terminals to state-sets
        ctn_states = collections.defaultdict(set)  # maps nonterminals to state-sets
        reduce_prods = {}  # maps follow-terminals to production indexes

        # Each state has three ways to advance.
        # - We can step over a terminal.
        # - We can step over a nonterminal.
        # - At the end of a production, we can reduce.
        # There is also a sort of "stepping in" effect for nonterminals, which
        # is achieved by the call to state_set_closure at the top of the loop.
        for state in state_set_closure(current_state_set):
            offset = state.offset
            nt, i, rhs = prods[state.prod_index]
            if offset < len(rhs):
                next_symbol = rhs[offset]
                if is_terminal(grammar, next_symbol):
                    if lookahead_contains(state.lookahead, next_symbol):
                        next_state = make_state(state.prod_index, offset + 1, None)
                        if next_state is not None:
                            shift_states[next_symbol].add(next_state)
                else:
                    # The next element is always a terminal or nonterminal,
                    # never an Optional (those are preprocessed out of the
                    # grammar) or a LookaheadRule (see make_state).
                    assert is_nt(grammar, next_symbol)

                    # We never reduce with a lookahead restriction still active,
                    # so the new state has no lookahead restrictions on it.
                    next_state = make_state(state.prod_index, offset + 1, lookahead=None)
                    if next_state is not None:
                        ctn_states[next_symbol].add(next_state)
            else:
                if state.lookahead is not None:
                    raise ValueError("invalid grammar: lookahead restriction still active "
                                     "at end of production " +
                                     production_to_str(grammar, nt, rhs))
                for t in follow[nt]:
                    if t in reduce_prods:
                        raise_reduce_reduce_conflict(t, reduce_prods[t], state.prod_index)
                    reduce_prods[t] = state.prod_index

        # Step 2. Turn that information into table data to drive the parser.
        action_row = {}
        for t, shift_state_set in shift_states.items():
            action_row[t] = get_state_set_index(shift_state_set)
        for t, prod_index in reduce_prods.items():
            nt, _, rhs = prods[prod_index]
            if t in action_row:
                raise ValueError("shift-reduce conflict when looking at {!r} after {}"
                                 .format(t, production_to_str(grammar, nt, rhs)))
            # Encode reduce actions as negative numbers.
            # Negative zero is the same as zero, hence the "- 1".
            action_row[t] = ACCEPT if nt == init_nt else -prod_index - 1
        ctn_row = {nt: get_state_set_index(ss) for nt, ss in ctn_states.items()}
        return action_row, ctn_row

    check(grammar)

    # Add an "init" nonterminal to the grammar. This nt is guaranteed to be
    # used in only one place, so that whenever it is reduced we know we're
    # done.
    grammar = clone_grammar(grammar)
    init_nt = gensym(grammar, goal)
    grammar[init_nt] = [
        [goal]
    ]

    grammar = make_epsilon_free_step_1(grammar)

    # Expand optional elements in the grammar. We replace each production that
    # contains an optional element with two productions: one with and one
    # without. This means the rest of the algorithm can ignore the possibility
    # of optional elements. But we keep the numbering of all the productions as
    # they appear in the original grammar (`prod_index` below).
    expanded_grammar = {}

    # Put all the productions in one big list, so each one has an index.
    # We will use the indices in the action table (as arguments to Reduce actions).
    prods = []

    # We'll use these tuples at run time when constructing AST nodes.
    reductions = []
    for nt in grammar:
        expanded_grammar[nt] = []
        for prod_index, rhs in enumerate(grammar[nt]):
            for expanded_rhs, removals in expand_optional_symbols(rhs):
                expanded_grammar[nt].append(expanded_rhs)
                prods.append((nt, prod_index, expanded_rhs))
                names = ["x" + str(i)
                         for i, e in enumerate(expanded_rhs)
                         if is_terminal(grammar, e) or is_nt(grammar, e)]
                names_with_none = names[:]
                for i in removals:
                    names_with_none.insert(i, "None")
                fn = ("lambda "
                      + ", ".join(names)
                      + ": ({!r}, {!r}, [".format(nt, prod_index)
                      + ", ".join(names_with_none)
                      + "])")
                reductions.append((nt, len(names), fn))
    grammar = expanded_grammar

    grammar = make_epsilon_free_step_2(grammar, goal)
    start = start_sets(grammar)

    # Note: this use of `init_nt` is a problem for adding multiple goal symbols.
    # Maybe we can just add a check at run time that we exited from the right place in the table...
    follow = follow_sets(grammar, init_nt)

    # A state set is a (frozen) set of pairs (production_index, offset_into_rhs).
    init_production_index = prods.index((init_nt, 0, [goal]))
    start_state = make_state(init_production_index, 0, None)
    if start_state is None:
        init_state_set = frozenset()
    else:
        init_state_set = frozenset({start_state})

    # We assign each reachable state set a number, and we keep a list of state
    # sets that have numbers but haven't been analyzed yet. When the list is
    # empty, we'll be done.
    visited_state_sets = {
        init_state_set: 0
    }
    todo = [init_state_set]

    actions = []
    ctns = []
    while todo:
        current_state_set = todo.pop(0)
        action_row, ctn_row = analyze_state_set(current_state_set)
        actions.append(action_row)
        ctns.append(ctn_row)


    # Write the parser.
    out.write("import pgen_runtime\n\n")
    out.write("actions = {}\n\n".format(pprint.pformat(actions, width=99)))
    out.write("ctns = {}\n\n".format(pprint.pformat(ctns, width=99)))
    out.write("reductions = [\n{}]\n\n"
              .format("".join("    ({!r}, {!r}, {}),\n".format(nt, length, reducer)
                              for nt, length, reducer in reductions)))
    out.write("parse = pgen_runtime.make_parse_fn(actions, ctns, reductions, 0)\n")


def compile(grammar, goal):
    out = io.StringIO()
    generate_parser(out, grammar, goal)
    scope = {}
    exec(out.getvalue(), scope)
    return scope['parse']


# *** Fun demo ****************************************************************

def main():
    grammar = example_grammar()

    class Tokens:
        def __init__(self, space_separated):
            self.tokens = space_separated.split()

        def peek(self):
            if len(self.tokens) == 0:
                return None
            else:
                next = self.tokens[0]
                if next.isdigit():
                    return "NUM"
                elif next.isalpha():
                    return "VAR"
                elif next in '+-*/()':
                    return next
                else:
                    raise ValueError("unexpected token {!r}".format(next))

        def take(self, k):
            if k is None:
                if self.tokens:
                    raise ValueError("expected end of input")
            else:
                assert self.peek() == k
                return self.tokens.pop(0)

    import io
    out = io.StringIO()
    generate_parser(out, grammar, 'expr')
    code = out.getvalue()
    print(code)
    print("----")

    sandbox = {}
    exec(code, sandbox)
    parse = sandbox['parse']

    while True:
        try:
            line = input('> ')
        except EOFError as _:
            break
        tokens = Tokens(line)
        try:
            result = parse(tokens)
        except Exception as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
        else:
            print(result)


if __name__ == '__main__':
    main()
