#!/usr/bin/env python3

"""gen.py - Fifth stab at a parser generator.

**Grammars.**
A grammar is a dictionary {str: [[symbol]]} mapping names of nonterminals to
lists of right-hand sides. Each right-hand side is a list of symbols. There
are several kinds of symbols; read the comments to learn more.

Instead of a list of right-hand sides, the value of a grammar entry may be a
function; see gen.Apply for details.

**Token streams.**
The user passes to each method an object representing the input sequence.
This object must support two methods:

*   `src.peek()` returns the kind of the next token, or `None` at the end of input.

*   `src.take(kind)` throws an exception if `src.peek() != kind`;
    otherwise, it removes the next token from the input stream and returns it.
    The special case `src.take(None)` checks that the input stream is empty:
    if so, it returns None; if not, it throws.

For very basic needs, see `lexer.LexicalGrammar`.
"""

import collections
import typing
import io
import sys
from ordered import OrderedSet, OrderedFrozenSet
from pgen_runtime import ERROR, ACCEPT
from lexer import SyntaxError


# *** What is a grammar? ******************************************************
#
# A grammar is a dictionary mapping nonterminal names to lists of right-hand
# sides. Each right-hand side (also called a "production") is a list whose
# elements can include terminals, nonterminals, Optional elements, LookaheadRules,
# and Apply elements (function calls).
#
# The most common elements are terminals and nonterminals, so a grammar usually
# looks something like this:
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


# Terminals are tokens that must appear verbatim in the input wherever they
# appear in the grammar, like the operators '+' '-' *' '/' and brackets '(' ')'
# in the example grammar.
def is_terminal(grammar, element):
    return type(element) is str and not is_nt(grammar, element)


# Nonterminals refer to other rules.
def is_nt(grammar, element):
    return type(element) is str and element in grammar


# Optional elements. These are expanded out before states are calculated,
# so the core of the algorithm never sees them.
Optional = collections.namedtuple("Optional", "inner")
Optional.__doc__ = """Optional(nt) matches either nothing or the given nt."""

def is_optional(element):
    return type(element) is Optional


# Function application. Function nonterminals are expanded out very early in
# the process, before states are calculated, so most of the algorithm doesn't
# see these. They're replaced with gensym names.
Apply = collections.namedtuple("Apply", "nt args")
Apply.__doc__ = """\
Apply(nt, (arg0, arg1, ...)) is a call to a nonterminal that's a function.

Each nonterminal in a grammar is defined by either a list of lists (its
productions) or a function that returns a list of lists.

To refer to the first kind of nonterminal in a right-hand-side, just use the
nonterminal's name. To use the second kind, we have to represent a function call
somehow; for that, use Apply.

The arguments are typically booleans. They can be whatever you want, but each
function nonterminal gets expanded into a set of productions, one for every
different argument tuple that is ever passed to it.
"""

def is_apply(element):
    """True if `element` smells like apples."""
    return type(element) is Apply


# Lookahead restrictions stay with us throughout the algorithm.
LookaheadRule = collections.namedtuple("LookaheadRule", "set positive")
LookaheadRule.__doc__ = """\
LookaheadRule(set, pos) imposes a lookahead restriction on whatever follows.

It never consumes any tokens itself. Instead, the right-hand side
[LookaheadRule(frozenset(['a', 'b']), False), 'Thing']
matches a Thing that does not start with the token `a` or `b`.
"""

def is_lookahead_rule(element):
    return type(element) is LookaheadRule


# A lookahead restriction really just specifies a set of allowed terminals.
#
# -   No lookahead restriction at all is equivalent to a rule specifying all terminals.
#
# -   A positive lookahead restriction explicitly lists all allowed tokens.
#
# -   A negative lookahead restriction instead specfies the set of all tokens
#     except a few.
#
def lookahead_contains(rule, t):
    """True if the given lookahead restriction `rule` allows the terminal `t`."""
    return (rule is None
            or (t in rule.set if rule.positive
                else t not in rule.set))


def lookahead_intersect(a, b):
    """Returns a single rule enforcing both `a` and `b`, allowing only terminals that pass both."""
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


# *** Basic grammar validity **************************************************

def check_grammar_types(grammar):
    """Throw if the given grammar is invalid.

    This only checks types. It doesn't check that the grammar is LR, that it's
    cycle-free, or any other nice properties.

    Normally, good Python style is never to check types but to plow ahead and
    let the language throw if the caller has erred. Here, the values are quite
    large, errors are likely, and if we don't check, the eventual TypeError
    doesn't usefully point to the location of the problem. So we check up front.

    (More justification: it's very sad to throw a bad error message while
    building a good one or debugging. Passing this predicate means a grammar
    can be used safely with `dump_grammar`, `rhs_to_str`, and so on.)
    """
    if not isinstance(grammar, typing.Mapping):
        raise TypeError("expected grammar dict, not {!r}".format(type(grammar).__name__))
    for nt, rhs_list_or_fn in grammar.items():
        if not isinstance(nt, str):
            raise TypeError("invalid grammar: expected string keys, got {!r}".format(nt))

        # A nonterminal maps to either a single function or (more typically) a
        # list of right-hand sides.  Function nonterminals can't be
        # type-checked here; we check the result after calling them.
        if not callable(rhs_list_or_fn):
            check_valid_rhs_list(nt, rhs_list_or_fn)


def check_valid_goals(grammar, goal_nts):
    for goal in goal_nts:
        if not is_nt(grammar, goal):
            raise ValueError("goal nonterminal {!r} is undefined".format(goal))


def check_valid_rhs_list(nt, rhs_list):
    if not isinstance(rhs_list, typing.Iterable):
        raise TypeError("invalid grammar: grammar[{!r}] should be a list of productions, not {!r}"
                        .format(nt, type(rhs_list).__name__))
    for i, rhs in enumerate(rhs_list):
        if not isinstance(rhs, typing.Iterable):
            raise TypeError("invalid grammar: grammar[{!r}][{}] should be a list of grammar symbols, not {!r}"
                            .format(nt, i, type(rhs).__name__))
        for e in rhs:
            if not isinstance(e, (str, Optional, LookaheadRule, Apply)):
                raise TypeError("invalid grammar: unrecognized element in production `grammar[{!r}][{}]`: {!r}"
                                .format(nt, i, e))


# *** Operations on grammars **************************************************

def fix(f, start):
    """Compute a fixed point of `f`, the hard way, starting from `start`."""
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
            for rhs, _r in expand_optional_symbols_in_rhs(source_rhs):
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


def check_lookahead_rules(grammar):
    """Check that no LookaheadRule appears at the end of a production (or before
    elements that can produce the empty string).

    If there are any offending lookahead rules, throw a ValueError.
    """

    check_cycle_free(grammar)
    for nt in grammar:
        for rhs_with_options in grammar[nt]:
            for rhs, _r in expand_optional_symbols_in_rhs(rhs_with_options):
                # XXX BUG: The next if-condition is insufficient, since it
                # fails to detect a lookahead restriction followed by a
                # nonterminal that can match the empty string.
                if rhs and is_lookahead_rule(rhs[-1]):
                    raise ValueError("invalid grammar: lookahead restriction at end of production: " +
                                     production_to_str(grammar, nt, rhs_with_options))


def clone_grammar(grammar):
    """ Return a deep copy of a grammar (which must contain no functions). """
    return {nt: [rhs[:] for rhs in rhs_list]
            for nt, rhs_list in grammar.items()}


def gensym(grammar, nt):
    """ Come up with a symbol name that's not already being used in the given grammar. """
    assert is_nt(grammar, nt)
    i = 0
    sym = nt
    while sym in grammar:
        i += 1
        sym = nt + "_" + str(i)
    return sym


def expand_function_nonterminals(grammar, goal_nts):
    """Replace function nonterminals with production lists."""

    # BUG: This use gensym() to create a bunch of new nonterminals, but without
    # any mapping back to the originals. Therefore the gensym names appear in
    # the parser output at run time. It would probably be OK to just use Apply
    # objects as nonterminal names.

    # Make dummy entries for everything in the grammar. gensym() needs them.
    result = {nt: None for nt in grammar}

    todo = list(goal_nts)
    assigned_names = {goal: goal for goal in todo}
    def expand_element(orig_e):
        e = orig_e
        opt = False
        while is_optional(e):
            opt = True
            e = e.inner

        if is_nt(grammar, e):
            target = e
        elif is_apply(e):
            target = e.nt
        else:
            return orig_e

        if target not in grammar:
            raise ValueError("invalid grammar: undefined nonterminal {} used in production {}"
                             .format(e, production_to_str(grammar, e, rhs)))
        if is_apply(e) and not callable(grammar[target]):
            raise ValueError("invalid grammar: {} is not a function, called in production {}"
                             .format(target, production_to_str(grammar, e, rhs)))

        if e not in assigned_names:
            if is_apply(e):
                name = gensym(result, e.nt)
                result[name] = None  # for the benefit of future gensym calls
            else:
                name = e
            assigned_names[e] = name
            todo.append(e)

        e = assigned_names[e]
        if opt:
            e = Optional(e)
        return e

    while todo:
        e = todo.pop(0)
        name = assigned_names[e]
        if is_nt(grammar, e):
            rhs_list = grammar[e]
        else:
            assert is_apply(e)
            rhs_list = grammar[e.nt](*e.args)
            check_valid_rhs_list("{}{!r}".format(e.nt, e.args), rhs_list)

        result[name] = [[expand_element(e) for e in rhs] for rhs in rhs_list]

    unreachable_keys = [nt for nt, rhs_list in result.items() if rhs_list is None]
    for key in unreachable_keys:
        del result[key]
    return result


def add_init_nonterminals(grammar, goals):
    """Add "init" nonterminals to the grammar. These are guaranteed to be used only
    as entry points, never on the rhs of any production, so that whenever they are
    reduced, we know we're done.

    Returns a pair: the modified grammar, and a dict {real_goal_nt:
    synthesized_init_nt, ...} mapping each nonterminal in `goals` to its new
    init_nt.
    """

    grammar = clone_grammar(grammar)
    init_nts = {}
    for goal_nt in goals:
        init_nt = gensym(grammar, goal_nt)
        grammar[init_nt] = [
            [goal_nt]
        ]
        assert goal_nt not in init_nts
        init_nts[goal_nt] = init_nt
    return grammar, init_nts


# *** Start sets and follow sets **********************************************

EMPTY = "(empty)"
END = None


def start_sets(grammar):
    """Compute the start sets for nonterminals in a grammar.

    A nonterminal's start set is the set of tokens that a match for that
    nonterminal may start with, plus EMPTY if it can match the empty string.
    """

    # How this works: Note that we can replace the words "match" and "start
    # with" in the definition above with more queries about start sets.
    #
    # 1.  A nonterminal's start set contains a terminal `t` if any of its
    #     productions contains either `t` or a nonterminal with `t` in *its*
    #     start set, preceded only by zero or more nonterminals that have EMPTY
    #     in *their* start sets. Plus:
    #
    # 2.  A nonterminal's start set contains EMPTY if any of its productions
    #     consists entirely of nonterminals that have EMPTY in *their* start
    #     sets.
    #
    # This definition is rather circular. We want the smallest collection of
    # start sets satisfying these rules, and we get that by iterating to a
    # fixed point.

    start = {nt: OrderedFrozenSet() for nt in grammar}
    done = False
    while not done:
        done = True
        for nt, rhs_list in grammar.items():
            # Compute start set for each `prod` based on `start` so far.
            # Could be incomplete, but we'll ratchet up as we iterate.
            nt_start = OrderedFrozenSet(t for rhs in rhs_list for t in seq_start(grammar, start, rhs))
            if nt_start != start[nt]:
                start[nt] = nt_start
                done = False
    return start


def seq_start(grammar, start, seq):
    """Compute the start set for a sequence of elements."""
    s = OrderedSet([EMPTY])
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
            return OrderedFrozenSet(future)
    return OrderedFrozenSet(s)


def make_start_set_cache(grammar, prods, start):
    """Compute start sets for all suffixes of productions in the grammar.

    Returns a list of lists `cache` such that
    `cache[n][i] == seq_start(grammar, start, prods[n][i:])`.

    (The cache is for speed, since seq_start was being called millions of times.)
    """
    start = start_sets(grammar)

    def suffix_start_list(rhs):
        sets = [OrderedFrozenSet([EMPTY])]
        for e in reversed(rhs):
            if is_terminal(grammar, e):
                s = OrderedFrozenSet([e])
            elif is_nt(grammar, e):
                s = start[e]
                if EMPTY in s:
                    s = OrderedFrozenSet((s - {EMPTY}) | sets[-1])
            else:
                assert is_lookahead_rule(e)
                if e.positive:
                    s = OrderedFrozenSet(sets[-1] & e.set)
                else:
                    s = OrderedFrozenSet(sets[-1] - e.set)
            assert isinstance(s, OrderedFrozenSet)
            assert s == seq_start(grammar, start, rhs[len(rhs) - len(sets):])
            sets.append(s)
        sets.reverse()
        assert sets == [seq_start(grammar, start, rhs[i:]) for i in range(len(rhs) + 1)]
        return sets

    return [suffix_start_list(prod.rhs) for prod in prods]


def follow_sets(grammar, prods_with_indexes_by_nt, start_set_cache, init_nts):
    """Compute all follow sets for nonterminals in a grammar.

    The follow set for a nonterminal `A`, as defined in the book, is "the set
    of terminals that can appear immediately to the right of `A` in some
    sentential form"; plus, "If `A` can be the rightmost symbol in some
    sentential form, then $ is in FOLLOW(A)."

    The `init_nts` argument is necessary to specify what a sentential form is,
    since sentential forms are partial derivations of a particular goal
    nonterminal.

    Returns a default-dictionary mapping nts to follow sets.
    """

    init_nts = list(init_nts)

    # Set of nonterminals already seen, including those we are in the middle of
    # analyzing. The algorithm starts at `goal` and walks all reachable
    # nonterminals, recursively.
    visited = set()

    # The results. By definition, nonterminals that are not reachable from the
    # goal nt have empty follow sets.
    follow = collections.defaultdict(OrderedSet)

    # If `(x, y) in subsumes_relation`, then x can appear at the end of a
    # production of y, and therefore follow[x] should be <= follow[y].
    # (We could maintain that invariant throughout, but at present we
    # brute-force iterate to a fixed point at the end.)
    subsumes_relation = OrderedSet()

    # `END` is $. It is, of course, in follow[each goal nonterminal]. It gets
    # into other nonterminals' follow sets through the subsumes relation.
    for init_nt in init_nts:
        follow[init_nt].add(END)

    def visit(nt):
        if nt in visited:
            return
        visited.add(nt)
        for prod_index, rhs in prods_with_indexes_by_nt[nt]:
            for i, symbol in enumerate(rhs):
                if is_nt(grammar, symbol):
                    visit(symbol)
                    after = start_set_cache[prod_index][i + 1]
                    if EMPTY in after:
                        after -= {EMPTY}
                        subsumes_relation.add((symbol, nt))
                    follow[symbol] |= after

    for nt in init_nts:
        visit(nt)

    # Now iterate to a fixed point on the subsumes relation.
    done = False
    while not done:
        done = True # optimistically
        for target, source in subsumes_relation:
            if follow[source] - follow[target]:
                follow[target] |= follow[source]
                done = False

    return follow


def expand_optional_symbols_in_rhs(rhs, start_index=0):
    """Expand a sequence that may contain optional symbols into sequences that don't.

    rhs is a list of symbols, possibly containing optional elements. This
    yields every list that can be made by replacing each optional element
    either with its .inner value, or with nothing.

    For example, `expand_optional_symbols_in_rhs(["if", Optional("else")])`
    yields the two sequences ["if"] and ["if", "else"].
    """

    for i in range(start_index, len(rhs)):
        if is_optional(rhs[i]):
            break
    else:
        yield rhs[start_index:], []
        return

    for expanded, r in expand_optional_symbols_in_rhs(rhs, i + 1):
        # without rhs[i]
        yield rhs[start_index:i] + expanded, [i] + r
        # with rhs[i]
        yield rhs[start_index:i] + [rhs[i].inner] + expanded, r


# At this point, lowered productions start getting farther from the original
# source.  We need to associate them with the original grammar in order to
# produce correct ouptut, so we use Prod values to represent productions.
#
# -   `nt` is the name of the nonterminal as it appears in the original grammar.
# -   `index` is the index of the source production, within nt's productions,
#     in the original grammar.
# -   `rhs` is the fully lowered/expanded right-hand-side of the production.
# -   `removals` is the list of indexes of elements in the original rhs
#     which were optional and are not present in this production.
#
# There may be many productions in a grammar that all have the same `nt` and `index`
# because they were all produced from the same source production.
Prod = collections.namedtuple("Prod", "nt index rhs removals")


def expand_all_optional_elements(grammar):
    """Expand optional elements in the grammar.

    We replace each production that contains an optional element with two
    productions: one with and one without. Downstream of this step, we can
    ignore the possibility of optional elements.
    """
    expanded_grammar = {}

    # Put all the productions in one big list, so each one has an index.
    # We will use the indices in the action table (as arguments to Reduce actions).
    prods = []
    prods_with_indexes_by_nt = collections.defaultdict(list)

    for nt in grammar:
        expanded_grammar[nt] = []
        for prod_index, rhs in enumerate(grammar[nt]):
            for expanded_rhs, removals in expand_optional_symbols_in_rhs(rhs):
                expanded_grammar[nt].append(expanded_rhs)
                prods.append(Prod(nt, prod_index, expanded_rhs, removals))
                prods_with_indexes_by_nt[nt].append((len(prods) - 1, expanded_rhs))

    return expanded_grammar, prods, prods_with_indexes_by_nt


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


def make_epsilon_free_step_2(grammar, goal_nts):
    """Return a clone of `grammar` with empty right-hand sides removed.

    All empty productions are removed except any for the goal nonterminals,
    so the grammar still recognizes the same language.
    """
    return {
        nt: [rhs for rhs in rhs_list if len(rhs) > 0 or nt in goal_nts]
        for nt, rhs_list in grammar.items()
    }


# *** The path algorithm ******************************************************

def find_path(start_set, successors, test):
    """Find a path from a value in `start_set` to a value that passes `test`.

    `start_set` is an iterable of "points". `successors` is a function mapping
    a point to an iterable of (edge, point) pairs. `test` is a predicate on
    points.  All points must support hashing; edges can be any value.

    Returns the shortest list `path` such that:
    - `path[0] in start_set`;
    - for every triplet `a, e, b` of adjacent elements in `path`
      starting with an even index, `(e, b) in successors(a)`;
    - `test(path[-1])`.

    If no such path exists, returns None.

    """

    # This implementation is long! I was tired when I wrote it.

    # Get started.
    links = {}
    todo = collections.deque()
    for p in start_set:
        if p not in links:
            links[p] = None
            if test(p):
                return [p]
            todo.append(p)

    # Iterate.
    found = False
    while todo:
        a = todo.popleft()
        for edge, b in successors(a):
            if b not in links:
                links[b] = a, edge
                if test(b):
                    found = True
                    todo.clear()
                    break
                todo.append(b)
    if not found:
        return None

    # Reconstruct how we got here.
    path = [b]
    while links[b] is not None:
        a, edge = links[b]
        path.append(edge)
        path.append(a)
        b = a
    path.reverse()
    return path


# *** How to dump stuff *******************************************************

def element_to_str(grammar, e):
    if is_nt(grammar, e):
        return e
    elif is_apply(e):
        return "{}[{}]".format(e.nt, ", ".join(repr(arg) for arg in e.args))
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
            s = '{' + repr(list(e.set))[1:-1] + '}'
        return "[lookahead {} {}]".format(op, s)
    else:
        return str(e)


def rhs_to_str(grammar, rhs):
    return " ".join(element_to_str(grammar, e) for e in rhs)


def production_to_str(grammar, nt, rhs):
    return "{} ::= {}".format(nt, rhs_to_str(grammar, rhs))


def dump_grammar(grammar):
    for nt, rhs_list in grammar.items():
        print(nt + " ::=")
        if callable(rhs_list):
            print("   ", repr(rhs_list))
        else:
            for rhs in rhs_list:
                if rhs:
                    print("   ", rhs_to_str(grammar, rhs))
                else:
                    print("   [empty]")
        print()


def lr_item_to_str(grammar, prods, item):
    prod = prods[item.prod_index]
    if item.lookahead is None:
        la = []
    else:
        la = [element_to_str(grammar, item.lookahead)]
    return "{} ::= {} >> {{{}}}".format(
        prod.nt,
        " ".join([element_to_str(grammar, e) for e in prod.rhs[:item.offset]]
                 + ["\N{MIDDLE DOT}"]
                 + la
                 + [element_to_str(grammar, e) for e in prod.rhs[item.offset:]]),
        ", ".join(
            "$" if t is None else element_to_str(grammar, t)
            for t in item.followed_by)
    )


def item_set_to_str(grammar, prods, item_set):
    return "{{{}}}".format(
        ",  ".join(lr_item_to_str(grammar, prods, item) for item in item_set)
    )


# *** Parser generation *******************************************************

# ## LR parsers: Why?
#
# Consider a single production `expr ::= expr "+" term` being parsed in a
# recursive descent parser. As we read the source left to right, our parser's
# internal state looks like this (marking our place with a dot):
#
#     expr ::= · expr "+" term
#     expr ::= expr · "+" term
#     expr ::= expr "+" · term
#     expr ::= expr "+" term ·
#
# As we go, we build an AST. First we parse an *expr* and temporarily set it
# aside. Then we expect to see a `+` operator. Then we parse a *term*. Then,
# having got to the end, we create an AST node for the whole addition
# expression.
#
# Since the grammar is nested, we really have a stack of these intermediate
# states.
#
# But how do we decide which production we should be matching? Often the first
# token just tells us: the `while` keyword means there's a `while` statement
# coming up. Grammars in which this is always the case are called LL(1). But
# while it's possible to wrangle *most* of the ES grammar into an LL(1) form,
# not everything works out. For example, here's the ES assignment syntax (much
# simplified):
#
#     assignment ::= sum
#     assignment ::= primitive "=" assignment
#     sum ::= primitive
#     sum ::= sum "+" primitive
#     primitive ::= VAR
#
# Note that the bogus assignment `a + b = c` doesn't parse because `a + b`
# isn't a primitive.
#
# Suppose we want to parse an expression, and the first token is `a`. We don't
# know yet which *assignment* production to use. So this grammar is not in
# LL(1).
#
#
# ## LR parsers: How
#
# An LR parser generator allows for a *superposition* of states.
# As we read `a = b + c`, our parser's internal state is like this
# (eliding a few steps, like how we recognize that `a` is a primitive):
#
#     current point in input  superposed parser state
#     ----------------------  -----------------------
#     · a = b + c             assignment ::= · sum
#                             assignment ::= · primitive "=" assignment
#
#       (Then, after recognizing that `a` is a *primitive*...)
#
#     a · = b + c             sum ::= primitive ·
#                             assignment ::= primitive · "=" assignment
#
#       (The next token, `=`, rules out the first alternative,
#       collapsing the waveform...)
#
#     a = · b + c             assignment ::= primitive "=" · assignment
#
#       (After recognizing that `b` is a primitive, we again have options:)
#
#     a = b · + c             sum ::= primitive ·
#                             assignment ::= primitive · "=" assignment
#
# And so on. We call each dotted production an "LR item", and the superposition
# of several LR items is called a "state".  (It is not meant to be clear yet
# just *how* the parser knows which rules might match.)
#
# Since the grammar is nested, we really have a stack of these parser state
# superpositions.
#
# The uncertainty in LR parsing means that code for an LR parser written by
# hand, in the style of recursive descent, would read like gibberish. What we
# can do instead is generate a parser table.


# An LRItem is a snapshot of progress through a single specific production.
#
# *   `prod_index` identifies the production. (Every production in the grammar
#     gets a unique index; see the loop that computes
#     prods_with_indexes_by_nt.)
#
# *   `offset` is the position of the cursor within the production.
#
# `lookahead` and `followed_by` are two totally different kinds of lookahead.
#
# *   `lookahead` is the LookaheadRule, if any, that applies to the immediately
#     upcoming input. It is present only if this LRItem is subject to a
#     `[lookahead]` restriction; otherwise it's None. These restrictions can't
#     extend beyond the end of a production, or else the grammar is invalid.
#     This is a hack and not part of any account of LR I've seen.
#
# *   `followed_by` is a completely different kind of lookahead restriction.
#     This is the kind of lookahead that is a central part of canonical LR
#     table generation.  It applies to the token *after* the whole current
#     production, so `followed_by` always applies to completely different and
#     later tokens than `lookahead`.  `followed_by` is a set of terminals; if
#     `None` is in this set, it means `END`, not that the LRItem is
#     unrestricted.
#
LRItem = collections.namedtuple("LRItem", "prod_index offset lookahead followed_by")


class PgenContext:
    """ The immutable part of the parser generator's data. """
    def __init__(self, grammar, init_nts, prods, prods_with_indexes_by_nt, start_set_cache, follow):
        self.grammar = grammar
        self.init_nts = OrderedFrozenSet(init_nts)
        self.prods = prods
        self.prods_with_indexes_by_nt = prods_with_indexes_by_nt
        self.start_set_cache = start_set_cache
        self.follow = follow

    def make_lr_item(self, *args, **kwargs):
        """Create an LRItem tuple and advance it past any lookahead rules.

        The main algorithm assumes that the "next element" in any LRItem is
        never a lookahead rule. We ensure that is true by processing lookahead
        elements before the LRItem is even exposed.

        We don't bother doing extra work here to eliminate lookahead
        restrictions that are redundant with what's coming up next in the
        grammar, like `[lookahead != NUM]` when the production is
        `name ::= IDENT`. We also don't eliminate items that can't match,
        like `name ::= IDENT` when we have `[lookahead not in {IDENT}]`.

        Such silly items can exist; but we would only care if it caused
        get_state_index to treat equivalent states as distinct. I haven't seen
        that happen for any grammar yet.

        """

        grammar = self.grammar
        prods = self.prods

        item = LRItem(*args, **kwargs)
        assert isinstance(item.followed_by, OrderedFrozenSet)
        rhs = prods[item.prod_index].rhs
        while item.offset < len(rhs) and is_lookahead_rule(rhs[item.offset]):
            item = item._replace(offset=item.offset + 1,
                                 lookahead=lookahead_intersect(item.lookahead, rhs[item.offset]))

        #if item.lookahead is not None:
        if False:  # this block is disabled for now; see comment
            # We want equivalent items to be ==, so the following code
            # canonicalizes lookahead rules, eliminates lookahead rules that
            # are redundant with the upcoming symbols in the rhs, and
            # eliminates items that (due to lookahead rules) won't match
            # anything.
            #
            # This sounds good in theory, and it does reduce the number of
            # LRItems we end up tracking, but I have not found an example where
            # it reduces the number of parser states. So this code is disabled
            # for now.

            expected = seq_start(grammar, start, rhs[item.offset:])
            if item.lookahead.positive:
                ok_set = expected & item.lookahead.set
            else:
                ok_set = expected - item.lookahead.set

            if len(ok_set) == 0:
                return None  # this item can't match anything
            elif ok_set == expected:
                look = None
            else:
                look = LookaheadRule(OrderedFrozenSet(ok_set), True)
            item = item._replace(lookahead=look)
        return item

    def raise_reduce_reduce_conflict(self, state, t, i, j):
        scenario_str = state.traceback()
        p1 = self.prods[i]
        p2 = self.prods[j]

        raise ValueError(
            "reduce-reduce conflict when looking at {} followed by {}\n"
            "can't decide whether to reduce with:\n"
            "    {}\n"
            "or with:\n"
            "    {}\n"
            .format(scenario_str, element_to_str(self.grammar, t),
                    production_to_str(self.grammar, p1.nt, p1.rhs),
                    production_to_str(self.grammar, p2.nt, p2.rhs)))

    def why_start(self, t, prod_index, offset):
        """ Yield a sequence of productions showing why `t in START(prods[prod_index][offset:])`.

        If `prods[prod_index][offset] is actually t, the sequence is empty.
        """
        # This code is garbage. I'm tired.
        # It depends on every symbol being either a terminal or nonterminal,
        # so it is actually pretty broken probably.
        assert t in self.start_set_cache[prod_index][offset]

        def successors(pair):
            prod_index, offset = pair
            rhs = self.prods[prod_index].rhs
            nt = rhs[offset]
            if not is_nt(self.grammar, nt):
                return
            for next_prod_index, next_rhs in self.prods_with_indexes_by_nt[nt]:
                if t in self.start_set_cache[next_prod_index][0]:
                    yield next_prod_index, (next_prod_index, 0)

        def done(pair):
            prod_index, offset = pair
            rhs = self.prods[prod_index].rhs
            return rhs[offset] == t

        path = find_path([(prod_index, offset)],
                         successors,
                         done)
        if path is None:  # oh, we found a bug. this was likely.
            return

        for prod_index in path[1::2]:
            prod = self.prods[prod_index]
            yield prod.nt, prod.rhs

    def why_follow(self, nt, t):
        """ Return a sequence of productions showing why the terminal t is in nt's follow set. """

        start_points = {}
        for prod_index, prod in enumerate(self.prods):
            nt1 = prod.nt
            rhs1 = prod.rhs
            for i in range(len(rhs1) - 1):
                if is_nt(self.grammar, rhs1[i]) and t in self.start_set_cache[prod_index][i + 1]:
                    start_points[rhs1[i]] = (prod_index, i + 1)

        def successors(nt):
            for prod_index, rhs in self.prods_with_indexes_by_nt[nt]:
                last = rhs[-1]
                if is_nt(self.grammar, last):
                    yield prod_index, last

        path = find_path(start_points.keys(), successors, lambda point: point == nt)

        # Yield productions showing how to produce `nt` in the right context.
        prod_index, offset = start_points[path[0]]
        prod = self.prods[prod_index]
        yield prod.nt, prod.rhs
        for index in path[1::2]:
            prod = self.prods[index]
            yield prod.nt, prod.rhs

        # Now show how the immediate next token can expand into something that starts with `t`.
        for xnt, xrhs in self.why_start(t, prod_index, offset):
            yield xnt, xrhs

    def raise_shift_reduce_conflict(self, state, t, shift_options, nt, rhs):
        assert shift_options
        assert t in self.follow[nt]
        grammar = self.grammar
        some_shift_option = next(iter(shift_options))
        shift_option_nt = self.prods[some_shift_option.prod_index].nt
        shift_option_nt_str = element_to_str(grammar, shift_option_nt)
        t_str = element_to_str(grammar, t)
        scenario_str = state.traceback()

        raise ValueError("shift-reduce conflict when looking at {} followed by {}\n"
                         "can't decide whether to shift into:\n"
                         "    {}\n"
                         "or reduce using:\n"
                         "    {}\n"
                         "\n"
                         "These productions show how {} can appear after {} (if we reduce):\n"
                         "{}"
                         .format(scenario_str,
                                 t_str,
                                 lr_item_to_str(grammar, self.prods, some_shift_option),
                                 production_to_str(grammar, nt, rhs),
                                 t_str,
                                 nt,
                                 "".join("    " + production_to_str(grammar, nt, rhs) + "\n"
                                         for nt, rhs in self.why_follow(nt, t))))


class State:
    """A parser state. A state is basically a set of LRItems.

    (For convenience, each State also has an attribute `self.context` that
    points to the PgenContext that has the grammar and various cached data; and
    an attribute `_debug_traceback` used in error messages. But for the most
    part, when we talk about a "state" we only care about the frozen set of
    LRItems in `self._lr_items`.)
    """

    __slots__ = ['context', '_lr_items', '_debug_traceback']

    def __init__(self, context, items, debug_traceback=None):
        self.context = context
        self._debug_traceback = debug_traceback

        # Consolidate similar items, to ensure that equivalent states have
        # equal _lr_items sets.
        a = collections.defaultdict(OrderedSet)
        for item in items:
            a[item.prod_index, item.offset, item.lookahead] |= item.followed_by
        self._lr_items = OrderedFrozenSet(LRItem(*k, OrderedFrozenSet(v)) for k, v in a.items())

    def __eq__(self, other):
        return self._lr_items == other._lr_items

    def __hash__(self):
        return hash(tuple(sorted(map(hash, self._lr_items))))

    def __str__(self):
        return "{{{}}}".format(
            ",  ".join(lr_item_to_str(self.context.grammar, self.context.prods, item)
                       for item in self._lr_items)
        )

    def closure(self):
        """Compute transitive closure of this state under left-calls.

        That is, return a superset of self that adds every item that's
        reachable from it by "stepping in" to nonterminals without consuming
        any tokens. Note that it's often possible to "step in" repeatedly.

        This is the only part of the system that makes items with lookahead
        restrictions.
        """
        context = self.context
        grammar = context.grammar
        prods = context.prods
        prods_with_indexes_by_nt = context.prods_with_indexes_by_nt
        start_set_cache = context.start_set_cache

        closure = OrderedSet(self._lr_items)
        closure_todo = collections.deque(self._lr_items)
        while closure_todo:
            item = closure_todo.popleft()
            rhs = prods[item.prod_index].rhs
            if item.offset < len(rhs):
                next_symbol = rhs[item.offset]
                if is_nt(grammar, next_symbol):
                    # Step in to each production for this nt.
                    for dest_prod_index, callee_rhs in prods_with_indexes_by_nt[next_symbol]:
                        # We may have rewritten the grammar just a tad since
                        # `prods` was built. (`prods` has to be built during the
                        # expansion of optional elements, but the grammar has
                        # to be modified a bit after that.) So, embarrassingly, we
                        # must now check that the production we just found is
                        # still in the grammar. XXX FIXME
                        if callee_rhs or callee_rhs in grammar[next_symbol]:
                            ## print("    Considering stepping from item {} into production {}"
                            ##       .format(lr_item_to_str(grammar, prods, item),
                            ##               production_to_str(grammar, next_symbol, callee_rhs)))
                            followers = specific_follow(start_set_cache,
                                                        item.prod_index, item.offset,
                                                        item.followed_by)
                            new_item = context.make_lr_item(dest_prod_index, 0, item.lookahead,
                                                            followers)
                            if new_item is not None and new_item not in closure:
                                closure.add(new_item)
                                closure_todo.append(new_item)
        return closure

    def traceback(self):
        """Return a list of terminals and nonterminals that could have gotten us here."""
        # _debug_traceback chains all the way back to the initial state.
        traceback = []
        ss = self
        while ss is not None:
            traceback.append(ss)
            ss = ss._debug_traceback
        assert next(iter(traceback[-1]._lr_items)).offset == 0
        del traceback[-1]
        traceback.reverse()

        scenario = []
        for ss in traceback:
            item = next(iter(ss._lr_items))
            prod = self.context.prods[item.prod_index]
            assert item.offset > 0
            scenario.append(prod.rhs[item.offset - 1])
        return rhs_to_str(self.context.grammar, scenario)


def specific_follow(start_set_cache, prod_id, offset, followed_by):
    """Return the set of tokens that might appear after the nonterminal rhs[offset],
    given that after `rhs` the next token will be a terminal in `followed_by`.
    """

    # First, which tokens might follow rhs[offset] *within* the rest of rhs?
    result = start_set_cache[prod_id][offset+1]
    if EMPTY in result:
        # The rest of rhs might be empty, so we might also see `followed_by`.
        result = OrderedSet(result)
        result.remove(EMPTY)
        result |= followed_by
    return OrderedFrozenSet(result)


def write_parser(out, grammar, states, actions, ctns, prods, init_state_map):
    out.write("import pgen_runtime\n\n")
    out.write("actions = [\n")
    for i, (state, row) in enumerate(zip(states, actions)):
        out.write("    # {}. {}\n".format(i, state.traceback() or "<empty>"))
        out.write("    " + repr(row) + ",\n")
        out.write("\n")
    out.write("]\n\n")
    out.write("ctns = [\n")
    for row in ctns:
        out.write("    " + repr(row) + ",\n")
    out.write("]\n\n")

    out.write("reductions = [\n")
    for prod in prods:
        names = ["x" + str(i)
                 for i, e in enumerate(prod.rhs)
                 if is_terminal(grammar, e) or is_nt(grammar, e)]
        names_with_none = names[:]
        for i in prod.removals:
            names_with_none.insert(i, "None")
        fn = ("lambda "
              + ", ".join(names)
              + ": ({!r}, {!r}, [".format(prod.nt, prod.index)
              + ", ".join(names_with_none)
              + "])")
        out.write("    ({!r}, {!r}, {}),\n".format(prod.nt, len(names), fn))
    out.write("]\n\n")

    for init_nt, index in init_state_map.items():
        out.write("parse_{} = pgen_runtime.make_parse_fn(actions, ctns, reductions, {})\n"
                  .format(init_nt, index))

TERMINAL_NAMES = {
    "{": "OpenBrace",
    "}": "CloseBrace",
    ";": "Semicolon",
    "?": "QuestionMark",
    "IDENT": "Identifier",
    "STR": "String",
}

def write_rust_parser(out, grammar, states, actions, ctns, prods, init_state_map):
    out.write("// THIS FILE IS AUTOGENERATED -- HAHAHAHA\n\n")

    out.write("use super::parser_runtime::{self, Node, ParserTables, TokenStream};\n\n")

    out.write("const ERROR: i64 = {};\n\n".format(hex(ERROR)))

    terminals = list(OrderedSet(t for row in actions for t in row))
    out.write("#[derive(Copy, Clone, Debug, PartialEq)]\n")
    out.write("pub enum TerminalId {\n")
    for i, t in enumerate(terminals):
        if t is None:
            name = "End"
        elif t in TERMINAL_NAMES:
            name = TERMINAL_NAMES[t]
        elif t.isalpha():
            name = t.capitalize()
        else:
            raise ValueError("mysterious token type: " + repr(t))
        out.write("    {} = {}, // {}\n".format(name, i, repr(t)))
    out.write("}\n\n")

    out.write("static ACTIONS: [i64; {}] = [\n".format(len(actions) * len(terminals)))
    for i, (state, row) in enumerate(zip(states, actions)):
        out.write("    // {}. {}\n".format(i, state.traceback() or "<empty>"))
        out.write("    {}\n".format(' '.join("{},".format(row.get(t, "ERROR")) for t in terminals)))
        if i < len(states) - 1:
            out.write("\n")
    out.write("];\n\n")

    nonterminals = list(OrderedSet(nt for row in ctns for nt in row))

    def to_camel_case(id):
        return ''.join(word.capitalize() for word in id.split('_'))

    seen = {}
    for nt in nonterminals:
        cc = to_camel_case(nt)
        if cc in seen:
            raise ValueError("{} and {} have the same camel-case spelling ({})".format(
                seen[cc], nt, cc))
        seen[cc] = nt

    def nt_node_variant(grammar, prod):
        name = to_camel_case(prod.nt)
        if len(grammar[prod.nt]) > 1:
            name += "P" + str(prod.index)
        return name

    seen = {}
    for prod in prods:
        if prod.nt in nonterminals and not prod.removals:
            name = nt_node_variant(grammar, prod)
            if name in seen:
                raise ValueError("Productions {} and {} have the same spelling ({})".format(
                    production_to_str(grammar, seen[name].nt, seen[name].rhs),
                    production_to_str(grammar, prod.nt, prod.rhs),
                    name))
            seen[name] = prod

    out.write("#[derive(Debug)]\n")
    out.write("pub enum NtNode {\n")
    for prod in prods:
        # Each production with an optional element removed uses the same
        # variant as the corresponding production where the optional element is
        # present.
        if prod.nt in nonterminals and not prod.removals:
            out.write("    // {}\n".format(production_to_str(grammar, prod.nt, prod.rhs)))
            name = nt_node_variant(grammar, prod)
            out.write("    {}({}),\n".format(name, ", ".join("Option<Node>" for v in prod.rhs)))
    out.write("}\n\n")

    out.write("#[derive(Clone, Copy, Debug, PartialEq)]\n")
    out.write("pub enum NonterminalId {\n")
    for i, nt in enumerate(nonterminals):
        out.write("    {} = {},\n".format(to_camel_case(nt), i))
    out.write("}\n\n")

    out.write("static GOTO: [usize; {}] = [\n".format(len(ctns) * len(nonterminals)))
    for row in ctns:
        out.write("    {}\n".format(' '.join("{},".format(row.get(nt, 0)) for nt in nonterminals)))
    out.write("];\n\n")

    out.write("fn reduce(prod: usize, stack: &mut Vec<Node>) -> NonterminalId {\n")
    out.write("    match prod {\n")
    for i, prod in enumerate(prods):
        # If prod.nt is not in nonterminals, that means it's a goal
        # nonterminal, only accepted, never reduced.
        if prod.nt in nonterminals:
            out.write("        {} => {{\n".format(i))
            out.write("            // {}\n".format(production_to_str(grammar, prod.nt, prod.rhs)))

            names_with_none = []
            for j, element in reversed(list(enumerate(prod.rhs))):
                if is_terminal(grammar, element) or is_nt(grammar, element):
                    out.write("            let x{} = stack.pop().unwrap();\n".format(j))
                    names_with_none.append("Some(x{})".format(j))

            names_with_none.reverse()
            for j in prod.removals:
                names_with_none.insert(j, "None")
            ntv = nt_node_variant(grammar, prod)
            out.write("            stack.push(Node::Nonterminal(Box::new(NtNode::{}({}))));\n".format(
                ntv,
                ", ".join(names_with_none)
            ))
            out.write("            NonterminalId::{}\n".format(to_camel_case(prod.nt)))
            out.write("        }\n")
    out.write('        _ => panic!("no such production: {}", prod),\n')
    out.write("    }\n")
    out.write("}\n\n")

    out.write(
        "static TABLES: ParserTables<'static> = ParserTables {\n" +
        "    state_count: {},\n".format(len(actions)) +
        "    action_table: &ACTIONS,\n" +
        "    action_width: {},\n".format(len(terminals)) +
        "    goto_table: &GOTO,\n" +
        "    goto_width: {},\n".format(len(nonterminals)) +
        "};\n\n"
    )

    for init_nt, index in init_state_map.items():
        out.write("pub fn parse_{}<In: TokenStream<Token=crate::ast::Token>>(\n".format(init_nt))
        out.write("    tokens: In,\n")
        out.write(") -> Result<Node, &'static str> {\n")
        out.write("    parser_runtime::parse(tokens, {}, &TABLES, reduce)\n".format(index))
        out.write("}\n\n")

class ParserGenerator:
    """ The core of the parser generation algorithm. """

    def __init__(self, context, prods, init_nt_map):
        # We assign each reachable state a number, and we keep a list of states
        # that have numbers but haven't been analyzed yet. When the list is empty,
        # we'll be done.
        self.todo = []
        self.visited_states = {}
        self.init_state_map = {}

        # Compute the start states.
        for goal_nt, init_nt in init_nt_map.items():
            init_prod_index = prods.index(Prod(init_nt, 0, [goal_nt], removals=[]))
            start_item = context.make_lr_item(init_prod_index,
                                              0,
                                              lookahead=None,
                                              followed_by=OrderedFrozenSet([END]))
            if start_item is None:
                init_state = State(context, [])
            else:
                init_state = State(context, [start_item])
            self.init_state_map[goal_nt] = self.get_state_index(init_state)

    def run(self):
        states = []
        actions = []
        ctns = []
        while self.todo:
            current_state = self.todo.pop(0)
            states.append(current_state)
            action_row, ctn_row = self.analyze_state(current_state)
            actions.append(action_row)
            ctns.append(ctn_row)
        return states, actions, ctns, self.init_state_map

    def get_state_index(self, successor):
        """ Get a number for a state, assigning a new number if needed. """
        assert isinstance(successor, State)
        visited_states = self.visited_states
        if successor in visited_states:
            return visited_states[successor]
        else:
            visited_states[successor] = state_index = len(visited_states)
            ## print("State #{} = {}"
            ##       .format(state_index, successor.closure()))
            self.todo.append(successor)
            return state_index

    def analyze_state(self, current_state):
        """Generate the LR parser table entry for a single state.

        This is done without iterating. But we sometimes need state-ids for
        states we haven't considered yet, so it calls self.get_state_index() --
        a side effect.
        """

        context = current_state.context
        grammar = context.grammar
        init_nts = context.init_nts
        prods = context.prods
        follow = context.follow

        #print("analyzing state {}".format(item_set_to_str(grammar, prods, current_state)))
        #print("  closure: {}".format(item_set_to_str(grammar, prods, current_state.closure())))

        # Step 1. Visit every item and list what we want to do for each
        # possible next token.
        shift_items = collections.defaultdict(OrderedSet)  # maps terminals to item-sets
        ctn_items = collections.defaultdict(OrderedSet)  # maps nonterminals to item-sets
        reduce_prods = {}  # maps follow-terminals to production indexes

        # Each item has three ways to advance.
        # - We can step over a terminal.
        # - We can step over a nonterminal.
        # - At the end of a production, we can reduce.
        # There is also a sort of "stepping in" effect for nonterminals, which
        # is achieved by the .closure() call at the top of the loop.
        for item in current_state.closure():
            offset = item.offset
            prod = prods[item.prod_index]
            nt = prod.nt
            i = prod.index
            rhs = prod.rhs
            if offset < len(rhs):
                next_symbol = rhs[offset]
                if is_terminal(grammar, next_symbol):
                    if lookahead_contains(item.lookahead, next_symbol):
                        next_item = context.make_lr_item(item.prod_index, offset + 1, None, item.followed_by)
                        if next_item is not None:
                            shift_items[next_symbol].add(next_item)
                else:
                    # The next element is always a terminal or nonterminal,
                    # never an Optional or Apply (those are preprocessed out of
                    # the grammar) or LookaheadRule (see make_lr_item).
                    assert is_nt(grammar, next_symbol)

                    # We never reduce with a lookahead restriction still
                    # active, so `lookahead=None` is appropriate.
                    next_item = context.make_lr_item(item.prod_index,
                                                     offset + 1,
                                                     lookahead=None,
                                                     followed_by=item.followed_by)
                    if next_item is not None:
                        ctn_items[next_symbol].add(next_item)
            else:
                if item.lookahead is not None:
                    # I think we could improve on this with canonical LR.
                    # The simplification in LALR might make it too weird though.
                    raise ValueError("invalid grammar: lookahead restriction still active "
                                     "at end of production " +
                                     production_to_str(grammar, nt, rhs))
                for t in item.followed_by:
                    if t in follow[nt]:
                        if t in reduce_prods:
                            context.raise_reduce_reduce_conflict(current_state, t, reduce_prods[t], item.prod_index)
                        reduce_prods[t] = item.prod_index

        # Step 2. Turn that information into table data to drive the parser.
        action_row = {}
        for t, shift_state in shift_items.items():
            shift_state = State(context, shift_state, current_state)  # freeze the set
            action_row[t] = self.get_state_index(shift_state)
        for t, prod_index in reduce_prods.items():
            prod = prods[prod_index]
            if t in action_row:
                context.raise_shift_reduce_conflict(current_state, t, shift_items[t], prod.nt, prod.rhs)
            # Encode reduce actions as negative numbers.
            # Negative zero is the same as zero, hence the "- 1".
            action_row[t] = ACCEPT if prod.nt in init_nts else -prod_index - 1
        ctn_row = {nt: self.get_state_index(State(context, ss, current_state))
                   for nt, ss in ctn_items.items()}
        return action_row, ctn_row

def generate_parser(out, grammar, goal_nts, target='python'):
    goal_nts = list(goal_nts)  # iterate this only once
    assert target in ('python', 'rust')

    # Step by step, we check the grammar and lower it to a more primitive form.
    check_grammar_types(grammar)
    check_valid_goals(grammar, goal_nts)
    grammar = expand_function_nonterminals(grammar, goal_nts)
    check_cycle_free(grammar)
    check_lookahead_rules(grammar)
    grammar, init_nt_map = add_init_nonterminals(grammar, goal_nts)
    init_nts = list(init_nt_map.values())
    grammar = make_epsilon_free_step_1(grammar)
    grammar, prods, prods_with_indexes_by_nt = expand_all_optional_elements(grammar)
    grammar = make_epsilon_free_step_2(grammar, goal_nts)

    # Now the grammar is in its final form. Compute information about it that
    # we can cache and use during the main part of the algorithm below.
    start = start_sets(grammar)
    start_set_cache = make_start_set_cache(grammar, prods, start)
    follow = follow_sets(grammar, prods_with_indexes_by_nt, start_set_cache, init_nts)
    context = PgenContext(grammar, init_nts, prods, prods_with_indexes_by_nt, start_set_cache, follow)

    # Run the core LR table generation algorithm.
    pgen = ParserGenerator(context, prods, init_nt_map)
    states, actions, ctns, init_state_map = pgen.run()

    # Finally, dump the output.
    if target == 'rust':
        write_rust_parser(out, grammar, states, actions, ctns, prods, init_state_map)
    else:
        write_parser(out, grammar, states, actions, ctns, prods, init_state_map)


class Parser:
    pass


def compile_multi(grammar, goals):
    goal_nts = list(goals)
    out = io.StringIO()
    generate_parser(out, grammar, goal_nts)
    scope = {}
    exec(out.getvalue(), scope)
    parser = Parser()
    for goal_nt in goal_nts:
        name = "parse_" + goal_nt
        setattr(parser, name, scope[name])
    return parser


def compile(grammar, goal):
    return getattr(compile_multi(grammar, [goal]), "parse_" + goal)


# *** Fun demo ****************************************************************

def demo():
    grammar = example_grammar()

    import lexer
    tokenize = lexer.LexicalGrammar("+ - * / ( )", NUM=r'0|[1-9][0-9]*', VAR=r'[_A-Za-z]\w+')

    import io
    out = io.StringIO()
    generate_parser(out, grammar, ['expr'])
    code = out.getvalue()
    print(code)
    print("----")

    sandbox = {}
    exec(code, sandbox)
    parse = sandbox['parse_expr']

    while True:
        try:
            line = input('> ')
        except EOFError as _:
            break
        try:
            result = parse(tokenize(line))
        except Exception as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
        else:
            print(result)


if __name__ == '__main__':
    demo()
