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
import pprint, textwrap

from pgen_runtime import is_terminal, is_nt, is_reduction, Reduction, ERROR, ACCEPT

def check(grammar):
    """Enforce three basic rules about the grammar.

    1.  Every nonterminal that appears in any production is defined.

    2.  The grammar contains no cycles.
        A cycle is a set of productions such that any
        nonterminal `q` has `q ==>+ q` (produces itself via at least one step).

    3.  No rule matches the empty string.

    If the grammar breaks any of these rules, throw.
    """

    # Maps names of nonterminals to one of:
    #     (missing) - we haven't seen this nt at all
    #     False - we are currently examining this nt for cycles
    #     True - we checked and this nt definitely is not in any cycles
    status = {}

    def check_nt(nt):
        s = status.get(nt)
        if s == True:
            return
        elif s == False:
            raise ValueError("invalid grammar: nonterminal {!r} has a cycle".format(nt))
        else:
            assert s is None
            status[nt] = False
            prods = grammar[nt]
            for prod in prods:
                for symbol in prod:
                    if is_nt(symbol) and symbol not in grammar:
                        raise ValueError("invalid grammar: nonterminal {!r} is used "
                                         "but not defined"
                                         .format(symbol))

                # Filter out reductions.
                prod = [symbol for symbol in prod if not is_reduction(symbol)]

                if len(prod) == 0:
                    raise ValueError("invalid grammar: nonterminal {!r} can match the empty string".format(nt))
                elif len(prod) == 1 and is_nt(prod[0]):
                    # Because we enforce rule 3 (no production can match the
                    # empty string), rule 2 is much easier to check: only
                    # productions consisting of exactly one nonterminal can be
                    # in cycles.
                    check_nt(prod[0])
            status[nt] = True

    for nt in grammar:
        check_nt(nt)


def gensym(grammar, nt):
    """ Come up with a symbol name that's not already being used in the given grammar. """
    assert is_nt(nt)
    while nt in grammar:
        nt += "_"
    return nt



def clone_grammar(grammar):
    return {nt: [prod[:] for prod in prods] for nt, prods in grammar.items()}


EMPTY = "(empty)"
END = None


def start(grammar, symbol):
    """Compute the start set for the given symbol.

    A symbol's start set is the set of tokens that a match for that symbol
    may start with, plus EMPTY if the symbol can match the empty string.
    """
    if is_terminal(symbol):
        # There is only one allowed match for a terminal.
        return {symbol}
    elif is_reduction(symbol):
        # Reductions always match the empty string.
        return {EMPTY}
    else:
        # Each nonterminal has a start set that depends on its productions.
        assert is_nt(symbol)
        return set.union(*(seq_start(grammar, prod)
                           for prod in grammar[symbol]))


def seq_start(grammar, seq):
    """Compute the start set for a sequence of symbols."""
    s = {EMPTY}
    for symbol in seq:
        if EMPTY not in s:  # preceding symbols never match the empty string
            break
        s.remove(EMPTY)
        s |= start(grammar, symbol)
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
                if is_nt(symbol):
                    visit(symbol)
                    after = seq_start(grammar, prod[i + 1:])
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


def dump_grammar(grammar):
    for nt, rules in sorted(grammar.items()):
        print(nt + " ::=")
        for s in rules:
            print("   ", s)


def generate_parser(out, grammar, goal):
    check(grammar)

    # Add an "init" nonterminal to the grammar. I don't know why this is
    # necessary but the book says to do it.
    grammar = clone_grammar(grammar)
    init_nt = gensym(grammar, goal)
    grammar[init_nt] = [
        [goal]
    ]

    # Put all the productions in one big list, so each one has an index.
    # We will use the indices in the action table (as arguments to Reduce actions).
    prods = [(nt, i, rhs) for nt in grammar for i, rhs in enumerate(grammar[nt])]

    # We'll use these tuples at run time when constructing AST nodes.
    reductions = [Reduction(nt, i, len(rhs)) for nt, i, rhs in prods]

    # Note: this use of `init_nt` is a problem for adding multiple goal symbols.
    # Maybe we can just add a check at run time that we exited from the right place in the table...
    follow = follow_sets(grammar, init_nt)

    # A state set is a (frozen) set of pairs (production_index, offset_into_rhs).
    init_production_index = prods.index((init_nt, 0, [goal]))
    init_state_set = frozenset({(init_production_index, 0)})

    def production_to_str(nt, rhs):
        return "{} ::= {}".format(nt, " ".join(rhs))

    def state_to_str(state):
        prod_index, offset = state
        nt, _i, rhs = prods[prod_index]
        return "{} ::= {}".format(
            nt,
            " ".join(rhs[:offset] + ["\N{MIDDLE DOT}"] + rhs[offset:])
        )

    def state_set_to_str(state_set):
        return "{{{}}}".format(
            ",  ".join(state_to_str(state) for state in state_set)
        )

    # We assign each reachable state set a number, and we keep a list of state
    # sets that have numbers but haven't been analyzed yet. When the list is
    # empty, we'll be done.
    visited_state_sets = {
        init_state_set: 0
    }
    todo = [init_state_set]

    def get_state_set_index(s):
        """ Get a number for a set of states, assigning a new number if needed. """
        successors = frozenset(s)
        if successors in visited_state_sets:
            return visited_state_sets[successors]
        else:
            visited_state_sets[successors] = state_index = len(visited_state_sets)
            #print("State #{} = {}".format(state_index, state_set_to_str(successors)))
            todo.append(successors)
            return state_index

    def state_set_closure(state_set):
        """ Compute transitive closure of the current state set under left-calls. """
        closure = set(state_set)
        closure_todo = list(state_set)
        while closure_todo:
            pair = closure_todo.pop(0)
            prod_index, offset = pair
            _nt, _i, rhs = prods[prod_index]
            if offset < len(rhs):
                next_symbol = rhs[offset]
                if is_nt(next_symbol):
                    for dest_prod_index, (dest_nt, _i, _rhs) in enumerate(prods):
                        if dest_nt == next_symbol:
                            pair = (dest_prod_index, 0)
                            if pair not in closure:
                                closure.add(pair)
                                closure_todo.append(pair)
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
            .format(t, production_to_str(nt1, rhs1), production_to_str(nt2, rhs2)))

    def analyze_state_set(current_state_set):
        #print("analyzing state set {}".format(state_set_to_str(current_state_set)))

        # Step 1. Visit every state and list what we want to do for each
        # possible next token.
        shift_states = collections.defaultdict(set)  # maps terminals to state-sets
        ctn_states = collections.defaultdict(set)  # maps nonterminals to state-sets
        reduce_prods = {}  # maps follow-terminals to production indexes

        for state in state_set_closure(current_state_set):
            prod_index, offset = state
            nt, i, rhs = prods[prod_index]
            if offset < len(rhs):
                next_symbol = rhs[offset]
                next_state = (prod_index, offset + 1)
                if is_terminal(next_symbol):
                    shift_states[next_symbol].add(next_state)
                else:
                    ctn_states[next_symbol].add(next_state)
            else:
                for t in follow[nt]:
                    if t in reduce_prods:
                        raise_reduce_reduce_conflict(t, reduce_prods[t], prod_index)
                    reduce_prods[t] = prod_index

        # Step 2. Turn that information into table data to drive the parser.
        action_row = {}
        for t, shift_state_set in shift_states.items():
            action_row[t] = get_state_set_index(shift_state_set)
        for t, prod_index in reduce_prods.items():
            nt, _, rhs = prods[prod_index]
            if t in action_row:
                raise ValueError("shift-reduce conflict when looking at {!r} after {}"
                                 .format(t, production_to_str(nt, rhs)))
            # Encode reduce actions as negative numbers.
            # Negative zero is the same as zero, hence the "- 1".
            action_row[t] = ACCEPT if nt == init_nt else -prod_index - 1
        ctn_row = {nt: get_state_set_index(ss) for nt, ss in ctn_states.items()}
        return action_row, ctn_row

    actions = []
    ctns = []
    while todo:
        current_state_set = todo.pop(0)
        action_row, ctn_row = analyze_state_set(current_state_set)
        actions.append(action_row)
        ctns.append(ctn_row)


    # Write the parser.
    out.write("import pgen_runtime\n"
              "from pgen_runtime import Reduction\n\n")
    out.write("actions = {}\n\n".format(pprint.pformat(actions, width=99)))
    out.write("ctns = {}\n\n".format(pprint.pformat(ctns, width=99)))
    out.write("reductions = {}\n\n".format(pprint.pformat(reductions, width=99)))
    out.write("parse = pgen_runtime.make_parse_fn(actions, ctns, reductions, 0)\n")

def main():
    grammar = {
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
