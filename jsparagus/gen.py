#!/usr/bin/env python3

"""gen.py - Fifth stab at a parser generator.

**Grammars.**
A grammar is a dictionary {str: [[symbol]]} mapping names of nonterminals to
lists of right-hand sides. Each right-hand side is a list of symbols. There
are several kinds of symbols; see grammar.py to learn more.

Instead of a list of right-hand sides, the value of a grammar entry may be a
function; see grammar.Nt for details.

**Token streams.**
The user passes to each method an object representing the input sequence.
This object must support two methods:

*   `src.peek()` returns the kind of the next token, or `None` at the end of
    input.

*   `src.take(kind)` throws an exception if `src.peek() != kind`;
    otherwise, it removes the next token from the input stream and returns it.
    The special case `src.take(None)` checks that the input stream is empty:
    if so, it returns None; if not, it throws.

For very basic needs, see `lexer.LexicalGrammar`.

"""

import collections
import io
import pickle
import sys
import itertools

from .ordered import OrderedSet, OrderedFrozenSet

from .grammar import (Grammar,
                      NtDef, Production, Some, CallMethod, InitNt,
                      is_concrete_element,
                      Optional, Exclude, Literal, UnicodeCategory, Nt, Var, ErrorSymbol,
                      LookaheadRule, lookahead_contains, lookahead_intersect)
from . import emit
from .runtime import ACCEPT, ErrorToken


# *** Operations on grammars **************************************************

def fix(f, start):
    """Compute a fixed point of `f`, the hard way, starting from `start`."""
    prev, current = start, f(start)
    while current != prev:
        prev, current = current, f(current)
    return current


def empty_nt_set(grammar):
    """Determine which nonterminals in `grammar` can produce the empty string.

    Return a dict {nt: expr} that maps each such nonterminal to the expr
    that should be evaluated when reducing the empty string to nt.
    So, for example, if we have a production

        a ::= b? c?  => CallMethod("a", [0, 1])

    then the resulting dictionary will contain the entry
    `("a", CallMethod("a", [None, None]))`.
    """

    empties = {}  # maps nts to reducers.

    def production_is_empty(nt, p):
        return all(isinstance(e, LookaheadRule)
                   or isinstance(e, Optional)
                   or (isinstance(e, Nt) and e in empties)
                   for e in p.body)

    def evaluate_reducer_with_empty_matches(p):
        # partial evaluation of p.reducer
        stack = [e for e in p.body if is_concrete_element(e)]

        def eval(expr):
            if expr is None:
                return None
            elif isinstance(expr, Some):
                return Some(eval(expr.inner))
            elif isinstance(expr, CallMethod):
                return CallMethod(
                    expr.method,
                    tuple(eval(arg_expr) for arg_expr in expr.args))
            elif isinstance(expr, int):
                e = stack[expr]
                if isinstance(e, Optional):
                    return None
                else:
                    assert isinstance(e, Nt)
                    return empties[e]
            elif expr == 'accept':
                # Hmm, this is not ideal! Maybe 'accept' needs to take an
                # argument so that the normal case is Accept(0) and this case
                # is Accept(eval(expr.args[0])).
                return 'accept'
            else:
                raise TypeError(
                    "internal error: unhandled reduce expression type {!r}"
                    .format(expr))

        return eval(p.reducer)

    done = False
    while not done:
        done = True
        for nt, nt_def in grammar.nonterminals.items():
            if nt not in empties:
                for p in nt_def.rhs_list:
                    if production_is_empty(nt, p):
                        if nt in empties:
                            raise ValueError(
                                "ambiguous grammar: multiple productions for "
                                "{!r} match the empty string"
                                .format(nt))
                        done = False
                        empties[nt] = evaluate_reducer_with_empty_matches(p)
    return empties


def check_cycle_free(grammar):
    """Throw an exception if any nonterminal in `grammar` produces itself
    via a cycle of 1 or more productions.
    """
    assert isinstance(grammar, Grammar)
    empties = empty_nt_set(grammar)

    # OK, first find out which nonterminals directly produce which other
    # nonterminals (after possibly erasing some optional/empty nts).
    direct_produces = {}
    for orig in grammar.nonterminals:
        direct_produces[orig] = set()
        for source_production in grammar.nonterminals[orig].rhs_list:
            for rhs, _r in expand_optional_symbols_in_rhs(source_production.body, grammar, empties):
                result = []
                all_possibly_empty_so_far = True
                # If we break out of the following loop, that means it turns
                # out that this production does not produce *any* strings that
                # are just a single nonterminal.
                for e in rhs:
                    if grammar.is_terminal(e):
                        break  # no good, this production contains a terminal
                    elif isinstance(e, Nt):
                        if e in empties:
                            if all_possibly_empty_so_far:
                                result.append(e)
                        else:
                            if not all_possibly_empty_so_far:
                                # Give up - we have 2+ nonterminals that can't
                                # be empty.
                                break
                            all_possibly_empty_so_far = False
                            result = [e]
                    elif isinstance(e, Optional):
                        if isinstance(e.inner, Nt):
                            # XXX FIXME BUG this can't be right
                            result.append(e.inner)
                    elif isinstance(e, Exclude):
                        if isinstance(e.inner, Nt):
                            result.append(e.inner)
                    elif isinstance(e, LookaheadRule):
                        # Ignore the restriction. We lose a little precision
                        # here; there could be a bug.
                        pass
                    elif isinstance(e, Literal):
                        if e.text != "":
                            # This production contains a non-empty character,
                            # therefore it cannot correspond to an empty cycle.
                            break
                    elif isinstance(e, UnicodeCategory):
                        # This production consume a class of character,
                        # therefore it cannot correspond to an empty cycle.
                        break
                    else:
                        # ErrorSymbol effectively matches the empty string
                        # (though only if nothing else matches).
                        assert isinstance(e, ErrorSymbol)
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

    for nt in grammar.nonterminals:
        if nt in produces[nt]:
            raise ValueError(
                "invalid grammar: nonterminal {} can produce itself"
                .format(nt))


def check_lookahead_rules(grammar):
    """Check that no LookaheadRule appears at the end of a production (or before
    elements that can produce the empty string).

    If there are any offending lookahead rules, throw a ValueError.
    """

    empties = empty_nt_set(grammar)

    check_cycle_free(grammar)
    for nt in grammar.nonterminals:
        for source_production in grammar.nonterminals[nt].rhs_list:
            body = source_production.body
            for rhs, _r in expand_optional_symbols_in_rhs(body, grammar, empties):
                # XXX BUG: The next if-condition is insufficient, since it
                # fails to detect a lookahead restriction followed by a
                # nonterminal that can match the empty string.
                if rhs and isinstance(rhs[-1], LookaheadRule):
                    raise ValueError(
                        "invalid grammar: lookahead restriction "
                        "at end of production: {}"
                        .format(grammar.production_to_str(nt, body)))


def expand_parameterized_nonterminals(grammar):
    """Replace parameterized nonterminals with specialized copies.

    For example, a single pair `nt_name: NtDef(params=('A', 'B'), ...)` in
    `grammar.nonterminals` will be replaced with (assuming A and B are boolean
    parameters) up to four pairs, each having an Nt object as the key and an
    NtDef with no parameters as the value.

    `grammar.nonterminals` must have string keys.

    Returns a new copy of `grammar` with Nt keys, whose NtDefs all have
    `nt_def.params == []`.
    """

    todo = collections.deque(grammar.goals())
    new_nonterminals = {}

    def expand(nt):
        """Expand grammar.nonterminals[nt](**args).

        Returns the expanded NtDef, which contains no conditional
        productions or Nt objects.
        """

        if nt.args is None:
            args_dict = None
        else:
            args_dict = dict(nt.args)

        def evaluate_arg(arg):
            if isinstance(arg, Var):
                return args_dict[arg.name]
            else:
                return arg

        def expand_element(e):
            if isinstance(e, Optional):
                return Optional(expand_element(e.inner))
            elif isinstance(e, Exclude):
                return Exclude(expand_element(e.inner), tuple(map(expand_element, e.exclusion_list)))
            elif isinstance(e, Nt):
                args = tuple((name, evaluate_arg(arg))
                             for name, arg in e.args)
                e = Nt(e.name, args)
                if e not in new_nonterminals:
                    todo.append(e)
                return e
            else:
                return e

        def expand_production(p):
            return p.copy_with(
                body=[expand_element(e) for e in p.body],
                condition=None)

        def expand_productions(nt_def):
            result = []
            for p in nt_def.rhs_list:
                if p.condition is None:
                    included = True
                else:
                    param, value = p.condition
                    included = (args_dict[param] == value)
                if included:
                    result.append(expand_production(p))
            return NtDef((), result, nt_def.type)

        nt_def = grammar.nonterminals[nt.name]
        assert tuple(name for name, value in nt.args) == nt_def.params
        return expand_productions(nt_def)

    while todo:
        nt = todo.popleft()
        if nt not in new_nonterminals:  # not already expanded
            new_nonterminals[nt] = expand(nt)

    return grammar.with_nonterminals(new_nonterminals)


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

    start = {nt: OrderedFrozenSet() for nt in grammar.nonterminals}
    done = False
    while not done:
        done = True
        for nt, nt_def in grammar.nonterminals.items():
            # Compute start set for each `prod` based on `start` so far.
            # Could be incomplete, but we'll ratchet up as we iterate.
            nt_start = OrderedFrozenSet(
                t for p in nt_def.rhs_list for t in seq_start(grammar, start, p.body))
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
        if grammar.is_terminal(e):
            s.add(e)
        elif isinstance(e, ErrorSymbol):
            s.add(ErrorToken)
        elif isinstance(e, Nt):
            s |= start[e]
        else:
            assert isinstance(e, LookaheadRule)
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

    (The cache is for speed, since seq_start was being called millions of
    times.)
    """

    def suffix_start_list(rhs):
        sets = [OrderedFrozenSet([EMPTY])]
        for e in reversed(rhs):
            if grammar.is_terminal(e):
                s = OrderedFrozenSet([e])
            elif isinstance(e, ErrorSymbol):
                s = OrderedFrozenSet([ErrorToken])
            elif isinstance(e, Nt):
                s = start[e]
                if EMPTY in s:
                    s = OrderedFrozenSet((s - {EMPTY}) | sets[-1])
            else:
                assert isinstance(e, LookaheadRule)
                if e.positive:
                    s = OrderedFrozenSet(sets[-1] & e.set)
                else:
                    s = OrderedFrozenSet(sets[-1] - e.set)
            assert isinstance(s, OrderedFrozenSet)
            assert s == seq_start(grammar, start, rhs[len(rhs) - len(sets):])
            sets.append(s)
        sets.reverse()
        assert sets == [seq_start(grammar, start, rhs[i:])
                        for i in range(len(rhs) + 1)]
        return sets

    return [suffix_start_list(prod.rhs) for prod in prods]


def follow_sets(grammar, prods_with_indexes_by_nt, start_set_cache):
    """Compute all follow sets for nonterminals in a grammar.

    The follow set for a nonterminal `A`, as defined in the book, is "the set
    of terminals that can appear immediately to the right of `A` in some
    sentential form"; plus, "If `A` can be the rightmost symbol in some
    sentential form, then $ is in FOLLOW(A)."

    Returns a default-dictionary mapping nts to follow sets.
    """

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
    for init_nt in grammar.init_nts:
        follow[init_nt].add(END)

    def visit(nt):
        if nt in visited:
            return
        visited.add(nt)
        for prod_index, rhs in prods_with_indexes_by_nt[nt]:
            for i, symbol in enumerate(rhs):
                if isinstance(symbol, Nt):
                    visit(symbol)
                    after = start_set_cache[prod_index][i + 1]
                    if EMPTY in after:
                        after -= {EMPTY}
                        subsumes_relation.add((symbol, nt))
                    follow[symbol] |= after

    for nt in grammar.init_nts:
        visit(nt)

    # Now iterate to a fixed point on the subsumes relation.
    done = False
    while not done:
        done = True  # optimistically
        for target, source in subsumes_relation:
            if follow[source] - follow[target]:
                follow[target] |= follow[source]
                done = False

    return follow


# *** Lowering ****************************************************************

# At this point, lowered productions start getting farther from the original
# source.  We need to associate them with the original grammar in order to
# produce correct output, so we use Prod values to represent productions.
#
# -   `nt` is the name of the nonterminal as it appears in the original
#     grammar.
#
# -   `index` is the index of the source production, within nt's productions,
#     in the original grammar.
#
# -   `rhs` is the fully lowered/expanded right-hand-side of the production.
#
# There may be many productions in a grammar that all have the same `nt` and
# `index` because they were all produced from the same source production.
Prod = collections.namedtuple("Prod", "nt index rhs reducer")


def expand_optional_symbols_in_rhs(rhs, grammar, empties, start_index=0):
    """Expand a sequence with optional symbols into sequences that have none.

    rhs is a list of symbols, possibly containing optional elements. This
    yields every list that can be made by replacing each optional element
    either with its .inner value, or with nothing.

    Each list is accompanied by the list of the indices of optional elements in
    `rhs` that were dropped.

    For example, `expand_optional_symbols_in_rhs(["if", Optional("else")])`
    yields the two pairs `(["if"], [1])` and `["if", "else"], []`.
    """

    for i in range(start_index, len(rhs)):
        e = rhs[i]
        if isinstance(e, Optional):
            if isinstance(e.inner, Nt) and e.inner in empties:
                # If this is already possibly-empty in the input grammar, it's an
                # error! The grammar is ambiguous.
                raise ValueError(
                    "ambiguous grammar: {} is ambiguous because {} can match "
                    "the empty string"
                    .format(grammar.element_to_str(e),
                            grammar.element_to_str(e.inner)))
            replacement = None
            break
        elif isinstance(e, Nt) and e in empties:
            replacement = empties[e]
            break
    else:
        yield rhs[start_index:], {}
        return

    for expanded, r in expand_optional_symbols_in_rhs(rhs, grammar, empties, i + 1):
        rhs_inner = rhs[i].inner if isinstance(rhs[i], Optional) else rhs[i]
        # without rhs[i]
        r2 = r.copy()
        r2[i] = replacement
        yield rhs[start_index:i] + expanded, r2
        # with rhs[i]
        yield rhs[start_index:i] + [rhs_inner] + expanded, r


def expand_all_optional_elements(grammar):
    """Expand optional elements in the grammar.

    We replace each production that contains an optional element with two
    productions: one with and one without. Downstream of this step, we can
    ignore the possibility of optional elements.
    """
    expanded_grammar = {}

    empties = empty_nt_set(grammar)

    # Put all the productions in one big list, so each one has an index. We
    # will use the indices in the action table (as reduce action payloads).
    prods = []
    prods_with_indexes_by_nt = collections.defaultdict(list)

    for nt, nt_def in grammar.nonterminals.items():
        prods_expanded = []
        for prod_index, p in enumerate(nt_def.rhs_list):
            # Aggravatingly, a reduce-expression that's an int is not
            # simply an offset into p.body. It only counts "concrete"
            # elements. Make a mapping for converting reduce-expressions to
            # offsets.
            reduce_expr_to_offset = [
                i
                for i, e in enumerate(p.body)
                if is_concrete_element(e)
            ]

            for pair in expand_optional_symbols_in_rhs(p.body, grammar, empties):
                expanded_rhs, removals = pair

                def adjust_reduce_expr(expr):
                    if isinstance(expr, int):
                        i = reduce_expr_to_offset[expr]
                        if i in removals:
                            return removals[i]
                        was_optional = isinstance(p.body[i], Optional)
                        expr -= sum(1 for r in removals if r < i)
                        if was_optional:
                            return Some(expr)
                        else:
                            return expr
                    elif expr is None:
                        return None
                    elif isinstance(expr, Some):
                        return Some(adjust_reduce_expr(expr.inner))
                    elif isinstance(expr, CallMethod):
                        return CallMethod(expr.method, [adjust_reduce_expr(arg)
                                                        for arg in expr.args])
                    elif expr == 'accept':
                        # doesn't need to be adjusted because 'accept' isn't
                        # turned into code downstream.
                        return 'accept'
                    else:
                        raise TypeError(
                            "internal error: unrecognized element {!r}"
                            .format(expr))

                adjusted_reducer = adjust_reduce_expr(p.reducer)
                prods_expanded.append(
                    Production(body=expanded_rhs,
                               reducer=adjusted_reducer))
                prods.append(Prod(nt, prod_index, expanded_rhs,
                                  adjusted_reducer))
                prods_with_indexes_by_nt[nt].append(
                    (len(prods) - 1, expanded_rhs))
        expanded_grammar[nt] = nt_def.with_rhs_list(prods_expanded)

    return (grammar.with_nonterminals(expanded_grammar),
            prods,
            prods_with_indexes_by_nt)


def remove_empty_productions(grammar):
    """Return a clone of `grammar` with empty right-hand sides removed.

    All empty productions are removed except any for the goal nonterminals,
    so the grammar still recognizes the same language.
    """
    goal_nts = set(grammar.goals())
    return grammar.with_nonterminals({
        nt: NtDef((), [p for p in nt_def.rhs_list
                       if len(p.body) > 0 or nt in goal_nts], nt_def.type)
        for nt, nt_def in grammar.nonterminals.items()
    })


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
# Since the grammar is nested, at run time we really have a stack of these
# intermediate states.
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
# An LR parser generator allows for a *superposition* of states. While parsing,
# we can sometimes have multiple productions at once that might match. It's
# like how in quantum theory, Schrödinger’s cat can tentatively be both alive
# and dead until decisive information is observed.
#
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
# Since the grammar is nested, at run time we'll have a stack of these parser
# state superpositions.
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
#     This implements the lookahead restrictions in the ECMAScript grammar.
#     It is not part of any account of LR I've seen.
#
# *   `followed_by` is a completely different kind of lookahead restriction.
#     This is the kind of lookahead that is a central part of canonical LR
#     table generation.  It applies to the token *after* the whole current
#     production, so `followed_by` always applies to completely different and
#     later tokens than `lookahead`.  `followed_by` is a set of terminals; if
#     `None` is in this set, it means `END`, not that the LRItem is
#     unrestricted.
#
LRItem = collections.namedtuple(
    "LRItem", "prod_index offset lookahead followed_by")


def assert_items_are_compatible(grammar, prods, items):
    """Assert that no two elements of `items` have conflicting history.

    All items in the same state must be produced by the same history,
    the same sequence of terminals and nonterminals.
    """
    def item_history(item):
        return [e
                for e in prods[item.prod_index].rhs[:item.offset]
                if not isinstance(e, LookaheadRule)]

    pairs = [(item, item_history(item)) for item in items]
    max_item, known_history = max(pairs, key=lambda pair: len(pair[1]))
    for item, history in pairs:
        assert grammar.compatible_sequences(history[:item.offset], known_history[-item.offset:]), \
            "incompatible LR items:\n    {}\n    {}\n".format(
                grammar.lr_item_to_str(prods, max_item),
                grammar.lr_item_to_str(prods, item))


class PgenContext:
    """ The immutable part of the parser generator's data. """

    def __init__(self, grammar, prods, prods_with_indexes_by_nt,
                 nt_start_sets, start_set_cache, follow):
        self.grammar = grammar
        self.prods = prods
        self.prods_with_indexes_by_nt = prods_with_indexes_by_nt
        self.nt_start_sets = nt_start_sets
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

        prods = self.prods

        item = LRItem(*args, **kwargs)
        assert isinstance(item.followed_by, OrderedFrozenSet)
        rhs = prods[item.prod_index].rhs
        while (item.offset < len(rhs)
               and isinstance(rhs[item.offset], LookaheadRule)):
            item = item._replace(
                offset=item.offset + 1,
                lookahead=lookahead_intersect(item.lookahead,
                                              rhs[item.offset]))

        # if item.lookahead is not None:
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

            expected = self.start_set_cache[item.prod_index][item.offset]
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
            .format(scenario_str, self.grammar.element_to_str(t),
                    self.grammar.production_to_str(p1.nt, p1.rhs),
                    self.grammar.production_to_str(p2.nt, p2.rhs)))

    def why_start(self, t, prod_index, offset):
        """Explain why `t in START(prods[prod_index][offset:])`.

        Yields a sequence of productions showing how the specified suffix can
        expand to something starting with `t`.

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
            if not isinstance(nt, Nt):
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
        """Explain why the terminal t is in nt's follow set.

        Yields a sequence of productions showing how a goal symbol can produce
        a string of terminals and nonterminals that contains nt followed by t.
        """

        start_points = {}
        for prod_index, prod in enumerate(self.prods):
            rhs1 = prod.rhs
            for i in range(len(rhs1) - 1):
                if (isinstance(rhs1[i], Nt)
                        and t in self.start_set_cache[prod_index][i + 1]):
                    start_points[rhs1[i]] = (prod_index, i + 1)

        def successors(nt):
            for prod_index, rhs in self.prods_with_indexes_by_nt[nt]:
                last = rhs[-1]
                if isinstance(last, Nt):
                    yield prod_index, last

        path = find_path(start_points.keys(), successors,
                         lambda point: point == nt)

        # Yield productions showing how to produce `nt` in the right context.
        prod_index, offset = start_points[path[0]]
        prod = self.prods[prod_index]
        yield prod.nt, prod.rhs
        for index in path[1::2]:
            prod = self.prods[index]
            yield prod.nt, prod.rhs

        # Now show how the immediate next token can expand into something that
        # starts with `t`.
        for xnt, xrhs in self.why_start(t, prod_index, offset):
            yield xnt, xrhs

    def raise_shift_reduce_conflict(self, state, t, shift_options, nt, rhs):
        assert shift_options
        assert t in self.follow[nt]
        grammar = self.grammar
        some_shift_option = next(iter(shift_options))
        t_str = grammar.element_to_str(t)
        scenario_str = state.traceback()
        productions = "".join(
            "    " + grammar.production_to_str(nt, rhs) + "\n"
            for nt, rhs in self.why_follow(nt, t))

        raise ValueError(
            "shift-reduce conflict when looking at {} followed by {}\n"
            "can't decide whether to shift into:\n"
            "    {}\n"
            "or reduce using:\n"
            "    {}\n"
            "\n"
            "These productions show how {} can appear after {} "
            "(if we reduce):\n"
            "{}"
            .format(
                scenario_str,
                t_str,
                grammar.lr_item_to_str(
                    self.prods, some_shift_option),
                grammar.production_to_str(nt, rhs),
                t_str,
                nt,
                productions))


class State:
    """A parser state. A state is basically a set of LRItems.

    During parser generation, states are annotated with attributes
    `.action_row` and `.ctn_row` that tell the actual parser what to do at run
    time. These will become rows of the parser tables.

    (For convenience, each State also has an attribute `self.context` that
    points to the PgenContext that has the grammar and various cached data; and
    an attribute `_debug_traceback` used in error messages. But for the most
    part, when we talk about a "state" we only care about the frozen set of
    LRItems in `self._lr_items`.)

    """

    __slots__ = [
        'context',
        '_lr_items',    # OrderedSet of LRItems, the actual content here
        '_debug_traceback',  # State from which this one was first reached
        'key',          # str, projection from _lr_items used to merge states
        '_hash',        # int, probably useless
        'action_row',   # output of analysis: {terminal: action}
        'ctn_row',      # output of analysis: {nonterminal: state_id}
        'error_code',   # error code to generate at runtime in this state
        'id'            # int, small unique id
    ]

    def __init__(self, context, items, debug_traceback=None):
        self.context = context
        self._debug_traceback = debug_traceback

        # Consolidate similar items, to ensure that equivalent states have
        # equal _lr_items sets.
        a = collections.defaultdict(OrderedSet)
        for item in items:
            a[item.prod_index, item.offset, item.lookahead] |= item.followed_by
        self._lr_items = OrderedFrozenSet(
            LRItem(*k, OrderedFrozenSet(v)) for k, v in a.items())

        # This state should be reused if another state is found that has all
        # the same items except with different .followed_by sets. This line of
        # code is what makes this an LALR parser generator rather than a
        # canonical LR parser generator.
        self.key = "".join(
            repr((item.prod_index, item.offset, item.lookahead)) + "\n"
            for item in sorted(self._lr_items))

        self._hash = hash(self.key)
        assert_items_are_compatible(
            context.grammar, context.prods, self._lr_items)

    def __eq__(self, other):
        return self.key == other.key

    def __hash__(self):
        return self._hash

    def __str__(self):
        return "{{{}}}".format(
            ",  ".join(
                self.context.grammar.lr_item_to_str(self.context.prods, item)
                for item in self._lr_items))

    def update(self, new_state):
        """Merge another State into self.

        This is called 0 or more times as we build out the graph of states.
        It's called each time an edge is found that points to `self`, except
        the first time. The caller has created a State object, `new_state`, but
        then found that this compatible State object already exists. Merge the
        two nodes. The caller discards `new_state` afterwards.

        Returns True if anything changed.
        """
        assert new_state.key == self.key
        assert len(self._lr_items) == len(new_state._lr_items)

        def item_key(item):
            return item.prod_index, item.offset, item.lookahead

        new_followed_by = {
            item_key(item): item.followed_by
            for item in new_state._lr_items
        }

        # If none of the new items adds any new followed_by symbols,
        # then there's nothing to update.
        if not any(new_followed_by[item_key(item)] - item.followed_by
                   for item in self._lr_items):
            return False

        # Really do the work of merging the two states.
        self._lr_items = OrderedFrozenSet(
            LRItem(*item_key(item),
                   item.followed_by | new_followed_by[item_key(item)])
            for item in self._lr_items
        )
        return True

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
                if isinstance(next_symbol, Nt):
                    # Step in to each production for this nt.
                    for pair in prods_with_indexes_by_nt[next_symbol]:
                        dest_prod_index, callee_rhs = pair

                        # We may have rewritten the grammar just a tad since
                        # `prods` was built. (`prods` has to be built during
                        # the expansion of optional elements, but the grammar
                        # has to be modified a bit after that.) So,
                        # embarrassingly, we must now check that the production
                        # we just found is still in the grammar. XXX FIXME
                        if (callee_rhs
                                or any(p.body == callee_rhs
                                       for p in grammar.nonterminals[next_symbol].rhs_list)):
                            # print(
                            #     "    Considering stepping from item {} "
                            #     "into production {}"
                            #     .format(
                            #         grammar.lr_item_to_str(prods, item),
                            #         grammar.production_to_str(next_symbol,
                            #                                   callee_rhs)))
                            followers = specific_follow(
                                start_set_cache,
                                item.prod_index,
                                item.offset,
                                item.followed_by)
                            new_item = context.make_lr_item(
                                dest_prod_index,
                                0,
                                item.lookahead,
                                followers)
                            if (new_item is not None
                                    and new_item not in closure):
                                closure.add(new_item)
                                closure_todo.append(new_item)
        return closure

    def analyze(self, get_state_index, *, verbose=False):
        """Generate the LR parser table entry for this state.

        This is done without iterating or recursing on states. But we sometimes
        need state-ids for states we haven't considered yet, so it calls
        get_state_index() -- a callback that can enqueue new states to be
        visited later.
        """

        context = self.context
        grammar = context.grammar
        prods = context.prods
        follow = context.follow
        nt_start_sets = context.nt_start_sets

        if verbose:
            print("State {}.".format(self.id))
            for item in self._lr_items:
                print("    " + grammar.lr_item_to_str(prods, item))
            print()

        # Step 1. Visit every item and list what we want to do for each
        # possible next token.
        shift_items = collections.defaultdict(
            OrderedSet)  # maps terminals to item-sets
        ctn_items = collections.defaultdict(
            OrderedSet)  # maps nonterminals to item-sets
        reduce_prods = {}  # maps follow-terminals to production indexes
        error_item = None  # item that contains an ErrorSymbol that could match here
        error_code = None  # that ErrorSymbol's error code

        def add_edge(table, item, e):
            """Make an edge from one item to a new item."""
            if e in grammar.synthetic_terminals:
                for rep in grammar.synthetic_terminals[e]:
                    add_edge(table, item, rep)
            else:
                # Check lookahead.
                if grammar.is_terminal(e):
                    if not lookahead_contains(item.lookahead, e):
                        return
                elif isinstance(e, Nt):
                    if not any(lookahead_contains(item.lookahead, t)
                               for t in nt_start_sets[e]):
                        return
                else:
                    assert e == ErrorToken

                next_item = context.make_lr_item(
                    item.prod_index,
                    offset + 1,
                    lookahead=None,
                    followed_by=item.followed_by)
                if next_item is not None:
                    table[e].add(next_item)

        # Each item has three ways to advance.
        # - We can step over a terminal.
        # - We can step over a nonterminal.
        # - At the end of a production, we can reduce.
        # There is also a sort of "stepping in" effect for nonterminals, which
        # is achieved by the .closure() call at the top of the loop.
        for item in self.closure():
            offset = item.offset
            prod = prods[item.prod_index]
            nt = prod.nt
            rhs = prod.rhs
            if offset < len(rhs):
                next_symbol = rhs[offset]
                if (grammar.is_terminal(next_symbol)
                        or isinstance(next_symbol, ErrorSymbol)):
                    if isinstance(next_symbol, ErrorSymbol):
                        t = ErrorToken
                        if error_item is None:
                            error_item = item
                            error_code = next_symbol.error_code
                        else:
                            context.raise_error_conflict(error_item, item)
                    else:
                        t = next_symbol

                    add_edge(shift_items, item, t)
                else:
                    # The next element is always a terminal or nonterminal,
                    # never an Optional (already preprocessed out of the
                    # grammar) or LookaheadRule (see make_lr_item).
                    assert isinstance(next_symbol, Nt)

                    # We never reduce with a lookahead restriction still
                    # active, so `check_lookahead=False` is appropriate.
                    add_edge(ctn_items, item, next_symbol)
            else:
                if item.lookahead is not None:
                    # I think we could improve on this with canonical LR. The
                    # simplification in LALR might make it too weird though.
                    raise ValueError(
                        "invalid grammar: lookahead restriction still active "
                        "at end of production {}"
                        .format(grammar.production_to_str(nt, rhs)))
                for t in grammar.expand_set_of_terminals(item.followed_by):
                    if t in grammar.expand_set_of_terminals(follow[nt]):
                        if t in reduce_prods:
                            context.raise_reduce_reduce_conflict(
                                self, t, reduce_prods[t], item.prod_index)
                        reduce_prods[t] = item.prod_index

        assert not any(t in grammar.synthetic_terminals for t in shift_items)
        assert not any(t in grammar.synthetic_terminals for t in ctn_items)

        # Step 2. Turn that information into table data to drive the parser.
        action_row = {}
        for t, shift_state in shift_items.items():
            shift_state = State(context, shift_state, self)  # freeze the set
            action_row[t] = get_state_index(shift_state)
        for t, prod_index in reduce_prods.items():
            prod = prods[prod_index]
            if t in action_row:
                if t == "else" and (len(prod.rhs) == 5
                                    and prod.rhs[0] == 'if'
                                    and prod.rhs[1] == '('
                                    and prod.rhs[3] == ')'):
                    # I can't believe I'm doing this, but manually resolve the
                    # shift-reduce conflict in favor of shifting.
                    assert action_row[t] >= 0, \
                        "the action we already have should be a shift"
                    continue  # leave it alone
                else:
                    context.raise_shift_reduce_conflict(
                        self, t, shift_items[t], prod.nt, prod.rhs)
            # Encode reduce actions as negative numbers.
            # Negative zero is the same as zero, hence the "- 1".
            action_row[t] = (
                ACCEPT if isinstance(prod.nt.name, InitNt)
                else -prod_index - 1)
        ctn_row = {nt: get_state_index(State(context, ss, self))
                   for nt, ss in ctn_items.items()}
        self.action_row = action_row
        self.ctn_row = ctn_row
        self.error_code = error_code

    def traceback_scenario(self):
        """Return example input that could have gotten us here.

        The result is a list of terminals and nonterminals.
        """
        # _debug_traceback chains all the way back to the initial state.
        traceback = []
        state = self
        while state is not None:
            traceback.append(state)
            state = state._debug_traceback
        assert next(iter(traceback[-1]._lr_items)).offset == 0
        del traceback[-1]
        traceback.reverse()

        scenario = []
        for state in traceback:
            item = next(iter(state._lr_items))
            prod = self.context.prods[item.prod_index]
            i = item.offset - 1
            assert i >= 0
            while isinstance(prod.rhs[i], LookaheadRule):
                i -= 1
                assert i >= 0
            scenario.append(prod.rhs[i])
        return scenario

    def traceback(self):
        """Return a string giving example input that could have gotten us here.

        This is for error messages.
        """
        return self.context.grammar.symbols_to_str(self.traceback_scenario())


def specific_follow(start_set_cache, prod_id, offset, followed_by):
    """Return the set of tokens that match after the nonterminal rhs[offset],
    given that after `rhs` the next token will be a terminal in `followed_by`.
    """

    # First, which tokens might follow rhs[offset] *within* the rest of rhs?
    result = start_set_cache[prod_id][offset + 1]
    if EMPTY in result:
        # The rest of rhs might be empty, so we might also see `followed_by`.
        result = OrderedSet(result)
        result.remove(EMPTY)
        result |= followed_by
    return OrderedFrozenSet(result)


def analyze_states(context, prods, *, verbose=False, progress=False):
    """The core of the parser generation algorithm."""

    # There is one state for each reachable set of LR items.
    # Each reachable state's id is its index in `states`.
    states = []
    states_by_key = {}
    todo = collections.deque()

    def get_state_index(successor):
        """ Get a number for a state, assigning a new number if needed. """
        assert isinstance(successor, State)
        state = states_by_key.get(successor.key)
        if state is not None:
            if state.update(successor):
                todo.append(state)
        else:
            state = successor
            state.id = len(states)
            states.append(state)
            states_by_key[state.key] = state
            todo.append(state)
        return state.id

    # Compute the start states.
    init_state_map = {}
    for init_nt in context.grammar.init_nts:
        init_prod_index = prods.index(
            Prod(init_nt, 0, [init_nt.name.goal], reducer="accept"))
        start_item = context.make_lr_item(init_prod_index,
                                          0,
                                          lookahead=None,
                                          followed_by=OrderedFrozenSet([END]))
        if start_item is None:
            init_state = State(context, [])
        else:
            init_state = State(context, [start_item])
        init_state_map[init_nt.name.goal] = get_state_index(init_state)

    # Turn the crank.
    def analyze_all_states():
        while todo:
            yield
            todo.popleft().analyze(get_state_index, verbose=verbose)

    consume(analyze_all_states(), progress)

    return ParserStates(context.grammar, prods, states, init_state_map)


def consume(iterator, progress):
    """Drain the iterator. If progress is true, print dots on stdout."""
    i = 0
    to_feed = str(i)
    try:
        for _ in iterator:
            if progress:
                if len(to_feed) > 0:
                    sys.stdout.write(to_feed[0])
                    to_feed = to_feed[1:]
                else:
                    sys.stdout.write(".")
                i += 1
                if i % 100 == 0:
                    sys.stdout.write("\n")
                    to_feed = str(i)
                sys.stdout.flush()
    finally:
        if progress and i != 0:
            sys.stdout.write("\n")
            sys.stdout.flush()


class ParserStates:
    """The output of LALR analysis of a Grammar."""

    def __init__(self, grammar, prods, states, init_state_map):
        self.grammar = grammar
        self.prods = prods
        self.states = states
        self.init_state_map = init_state_map

    def save(self, filename):
        with open(filename, 'wb') as f:
            pickle.dump(self, f)

    @classmethod
    def load(cls, filename):
        with open(filename, 'rb') as f:
            obj = pickle.load(f)
            if len(f.read()) != 0:
                raise ValueError("file has unexpected extra bytes at end")
        if not isinstance(obj, cls):
            raise TypeError("file contains wrong kind of object: expected {}, got {}"
                            .format(cls.__name__, obj.__class__.__name__))
        return obj


class CanonicalGrammar:
    def __init__(self, grammar, old = False):
        assert isinstance(grammar, Grammar)

        # Step by step, we check the grammar and lower it to a more primitive form.
        grammar = expand_parameterized_nonterminals(grammar)
        check_cycle_free(grammar)
        check_lookahead_rules(grammar)
        grammar, prods, prods_with_indexes_by_nt = expand_all_optional_elements(
            grammar)
        self.prods = prods
        self.prods_with_indexes_nt = prods_with_indexes_by_nt

        self.grammar = remove_empty_productions(grammar)


class StateAndTransitions:
    """This is one state of the parse table, which has transitions based on
    terminals (text), non-terminals (grammar rules) and epsilon (reduce).

    In this model epsilon transitions are used to represent code to be executed
    such as reduce actions and any others actions.
    """

    __slots__ = [
        "index",        # Numerical index of this state.
        "terminals",    # Terminals map.
        "nonterminals", # Non-terminals map.
        "epsilon",      # List of epsilon transitions with associated actions.
        "locations",    # Ordered set of LRItems (grammar position).
    ]

    def __init__(self, index):
        self.index = index
        self.terminals = {}
        self.nonterminals = {}
        self.epsilon = []
        self.locations = None

    def is_inconsistent(self):
        "Returns True if the state transitions are inconsistent."
        if len(self.terminals) + len(self.nonterminals) > 0 and len(self.epsilon) > 0:
            return True
        elif len(self.epsilon) > 1:
            if any([ not k.is_unordered_condition() for k, s in self.epsilon ]):
                return True
        return False

    def edges(self):
        for k, s in self.terminals:
            yield (k, s)
        for k, s in self.nonterminals:
            yield (k, s)
        for k, s in self.epsilon:
            yield (k, s)

class Action:
    __slots__ = [
        "read",    # Set of trait names which are consumed by this action.
        "write",   # Set of trait names which are mutated by this action.
    ]

    def __init__(self, read, write):
        assert isinstance(read, list)
        assert isinstance(write, list)
        self.read = read
        self.write = write

    def is_unordered_condition(self):
        "Unordered condition, which accept or not to reach the next state."
        return False

    def __eq__(self, other):
        if self.__class__ != other.__class__:
            return False
        if sorted(self.read) != sorted(other.read):
            return False
        if sorted(self.write) != sorted(other.write):
            return False
        for s in self.__slots__:
            if getattr(self, s) != getattr(other, s):
                return False
        return True

    def __hash__(self, other):
        return hash(tuple(
            [self.__class__, self.kind, "rd"] + self.read + ["wr"] + self.write +
            [repr(getattr(self, s)) for s in self.__slots__]
        ))

class Reduce(Action):
    """Define a reduce operation which pops N elements of he stack and pushes one
    non-terminal. The replay attribute of a reduce action corresponds to the
    number of stack elements which would have to be popped and pushed again
    using the parser table after reducing this operation. """
    __slots__ = 'nt', 'replay', 'popped'
    def __init__(self, nt, pop, replay = 0):
        super().__init__([], ["nt_" + nt])
        self.nt = nt    # Non-terminal which is reduced
        self.pop = pop  # Number of stack elements which should be replayed.
        self.replay = replay # Number of popped elements to match the production.

class Lookahead(Action):
    """Define a Lookahead assertion which is meant to either accept or reject
    sequences of terminal/non-terminals sequences."""
    __slots__ = 'sequences', 'accept'
    def __init__(self, sequences, accept):
        super().__init__([], [])
        self.sequences = sequences,
        self.accept = accept
    def is_unordered_condition(self):
        return True

class FilterStack(Action):
    """Check whether the stack contains a given state at the given offset. This is
    used for checking the context of a rule without loosing the sharing of LR0
    parse table. """
    __slots__ = 'state', 'offset'
    def __init__(self, state, offset):
        super().__init__([], [])
        self.state = state,
        self.offset = offset
    def is_unordered_condition(self):
        return True

class FilterFlag(Action):
    """Define a new action which check g has th[ e for e in e expected value. If so
    the Error state. """
    __slots__ = 'flag', 'accept'
    def __init__(self, flag, accept):
        super().__init__(["flag_" + flag], [])
        self.flag = flag,
        self.accept = accept
    def is_unordered_condition(self):
        return True

class PushFlag(Action):
    """Define an action which pushes a flag as set or unset on the flag bit stack."""
    __slots__ = 'flag', 'value'
    def __init__(self, flag, value):
        super().__init__([], ["flag_" + flag])
        self.flag = flag,
        self.value = value

class PopFlag(Action):
    """Define an action which pops a flag from the flag bit stack."""
    __slots__ = ['flag']
    def __init__(self, flag):
        super().__init__(["flag_" + flag], ["flag_" + flag])
        self.flag = flag,

class FunCall(Action):
    """Define a call method operation which reads N elements of he stack and
    pushpathne non-terminal. The replay attribute of a reduce action correspond
    to the number of stack elements which would have to be popped and pushed
    again using the parser table after reducing this operation. """
    __slots__ = 'method', 'offset', 'read_len'
    def __init__(self, method, alias_read, alias_write, read_len, offset = 0):
        super().__init__(alias_read, alias_write)
        self.method = method     # Method and argument to be read for calling it.
        self.offset = offset     # Offset to add to each argument offset.
        self.read_len = read_len # Range of numbers which can be read.

def on_stack(grammar, term):
    """Returns whether an element of a production is consuming stack space or
    not."""
    if isinstance(term, Nt):
        return True
    elif grammar.is_terminal(term):
        return True
    elif isinstance(term, LookaheadRule):
        return False
    raise ValueError(term)

class LR0Generator:
    """Provide a way to iterate over the grammar, given a set of LR items."""
    __slots__ = [
        "grammar",
        "lr_items",
        "key",
        "_hash",
    ]

    def __init__(self, grammar, lr_items = []):
        self.grammar = grammar
        self.lr_items = OrderedFrozenSet(lr_items)
        # This is used to reuse states which have already been encoded.
        self.key = "".join(repr((item.prod_index, item.offset)) + "\n"
                           for item in sorted(self.lr_items))
        self._hash = hash(self.key)

    def __eq__(self, other):
        return self.key == other.key

    def __hash__(self, other):
        return self._hash

    @staticmethod
    def start(grammar, nt):
        assert isinstance(grammar, CanonicalGrammar)
        assert isinstance(grammar.grammar, Grammar)
        lr_items = []
        for prod in grammar.prods_with_indexes_by_nt[nt]:
            assert isinstance(prod, Production)
            lr_items.append(LRItem(
                prod_index = prod.index,
                offset = 0,
                lookahead = [],
                followed_by = [],
            ))
        return LR0Generator(grammar, lr_items)

    def transitions(self):
        """Returns the dictionary which maps the state transitions with the next
        LR0Generators. This can be used to generate the states and the
        transitions between the states of an LR0 parse table."""
        followed_by = defaultdict(lambda [])
        for lr_item in self.lr_items:
            self.next_item(lr_item, followed_by)

        for k, lr_items in followed_by:
            followed_by[k] = LR0Generator(grammar, lr_items)
        return followed_by

    def item_transitions(self, lr_item, followed_by):
        """Given one LR Item, register all the transitions and LR Items reachables
        through these transitions."""
        prod = self.grammar.prod[lr_item.prod_index]

        # Read the term located at the offset in the production.
        if len(prod.rhs) > lr_item.offset:
            term = prod.rhs[lr_item.offset]
        elif len(prod.rhs) == lr_item.offset:
            # Add the reduce operation as a state transition in the generated
            # automaton. (TODO: this supposed that the canonical form did not
            # move the reduce action to be part of the production)
            pop = len([e for e in prod.rhs if on_stack(self.grammar.grammar, e)])
            term = Reduce(prod.nt, pop)
        else:
            # No edges after the reduce operation.
            return

        if isinstance(term, LookaheadRule):
            term = Lookahead(term.set, term.positive)

        # Add terminals, non-terminals and lookahead actions, as transitions to
        # the next LR Item.
        new_transition = term not in followed_by
        followed_by[term].append(LRItem(
            prod_index = lr_item.prod_index,
            offset = lr_item.offset + 1,
            lookahead = [],
            followed_by = [],
        ))

        # If the term is a non-terminal, then recursively add transitions from
        # the beginning of all the productions which are matching this
        # non-terminal.
        #
        # Only do it once per non-terminal to avoid infinite recursion on
        # left-recursive grammars.
        if isinstance(term, Nt) and new_transition:
            for prod in self.grammar.prods_with_indexes_by_nt[term]:
                assert isinstance(prod, Production)
                self.next_item(LRItem(
                    prod_index = prod.index,
                    offset = 0,
                    lookahead = [],
                    followed_by = [],
                ), followed_by)

class ParseTable:
    """The parser can be represented as a matrix of state transitions where on one
    side we have the current state, and on the other we have the expected
    terminal, non-terminal or epsilon transition.

            a   b   c   A   B   C   #1   #2   #3
          +---+---+---+---+---+---+----+----+----+
      s1  |   |   |   |   |   |   |    |    |    |
      s2  |   |   |   |   |   |   |    |    |    |
      s3  |   |   |   |   |   |   |    |    |    |
       .  |   |   |   |   |   |   |    |    |    |
       .  |   |   |   |   |   |   |    |    |    |
       .  |   |   |   |   |   |   |    |    |    |
      s67 |   |   |   |   |   |   |    |    |    |
      s68 |   |   |   |   |   |   |    |    |    |
      s69 |   |   |   |   |   |   |    |    |    |
          +---+---+---+---+---+---+----+----+----+

    The terminals `a` are the token which are read from the input. The
    non-terminals `A` are the token which are pushed by the reduce actions of
    the epsilon transitions. The epsilon transitions `#1` are the actions which
    have to be executed as code by the parser.

    A parse table is inconsistent if there is any state which has an epsilon
    transitions and terminals/non-terminals transitions (shift-reduce
    conflict), or a state with more than one epsilon transitions (reduce-reduce
    conflict). This is equivalent to having a non deterministic state machine.

    """

    __slots__ = [
        # Map of actions identifier to the corresponding object.
        "actions",
        # Map of state identifier to the corresponding object.
        "states",
        # List of states which are the entry point of the state machine.
        "goals",
        # List of terminals.
        "terminals",
        # List of non-terminals.
        "nonterminals",
        # Map non-terminals to the sequences of states used to produce these
        # non-terminals.
        "reduce_paths",
    ]

    def __init__(self, grammar, verbose = False, progress = False):
        self.actions = []
        self.states = []
        self.goals = []
        self.terminals = []
        self.nonterminals = []
        self.reduce_paths = {}
        self.create_lr0_table(grammar, verbose, progress)
        self.compute_reduce_paths(verbose, progress)
        self.fix_inconsistent_table(verbose, progress)

    def is_inconsistent(self):
        "Returns True if the grammar contains any inconsistent state."
        for s in self.states:
            if s.is_inconsistent():
                return True
        return False

    def new_state(self):
        "Create a new state with a given index."
        index = len(self.states)
        state = StateAndTransitions(index)
        self.states.append(state)
        return state

    def create_lr0_table(self, grammar, verbose, progress):
        if verbose:
            print("Create LR(0) parse table.")
        assert isinstance(grammar, CanonicalGrammar)
        assert isinstance(grammar.grammar, Grammar)

        goals = self.grammar.grammar.goals()
        goals = [ nt, self.new_state() for nt in goals ]

        # Ensure that identical set of LR ITems will map to the same state.
        lr0_cache = {}
        # Temporarily record tuples of (LR0Generator, StateAndTransition)
        # objects used for visiting the grammar.
        todo = collections.deque()
        # Record the starting goals in the todo list.
        for nt, s in goals:
            it = LR0Generator.start(grammar, nt)
            s.locations = it.lr_items
            lr0_cache[it] = s
            todo.append((it, s))

        # Iterate the grammar with sets of LR Items abstracted by the
        # LR0Generator, and create new states in the parse table as long as new
        # sets of LR Items are discovered.
        def visit_grammar():
            while todo:
                yield # progress bar.
                # TODO: Compare stack / queue, for the traversal of the states.
                s_it, s = todo.popleft()
                for k, sk_it in s_it.next():
                    if sk_it in lr0_cache:
                        sk = state in lr0_cache[sk_it]
                    else:
                        sk = self.new_state()
                        sk.locations = sk_it.lr_items
                        lr0_cache[it] = sk
                        todo.append((sk_it, sk))

                    # Add the edge from s to sk with k.
                    if isinstance(k, Action):
                        s.epsilon.append((k, sk))
                    elif isinstance(k, Nt):
                        s.nonterminals[k] = sk
                    else:
                        s.terminals[k] = sk
        consume(visit_grammar(), progress)

    def compute_reduce_paths(self, verbose, progress):
        """Reduce actions are currently ending with bogus states which are unreachable.
        However, for solving inconsistencies, we might need more lookahead
        terms to disambiguate state machine.

        This function computes the list of states to which a reduce function
        might jump back to after being executed."""
        if verbose:
            print("Compute reduce paths.")

        todo = collections.deque()
        for s in self.goals:
            todo.append([s])

        def visit_table():
            while todo:
                yield # progress bar.
                # TODO: Compare stack / queue, for the traversal of the states.
                shifted = todo.popleft()
                state = self.states[shifted[-1]]
                for term, to in state.edges():
                    if to in shifted:
                        # We already traversed this state before.
                        continue
                    if isinstance(term, Action):
                        if isinstance(term, Reduce):
                            assert term.replay == 0
                            # NOTE: the first state is the state before
                            # shifting the first terminals/non-terminals of the
                            # Nt production.
                            incoming = shifted[-1 - term.pop:]
                            self.reduce_paths[term.nt].add(incoming)
                        else:
                            todo.append(shifted[:-1] + [to])
                    else:
                        todo.append(shifted + [to])
        consume(visit_table(), progress)

    # To fix inconsistencies of the grammar, we have to traverse the grammar
    # both forward by using the lookahead and backward by using the parser's
    # emulated stack recovered from reduce actions.
    #
    # To do so we define the notion of abstract parser state (APS), which is a
    # tuple which represent the known state of the parser, as:
    #   (stack, shift, lookahead, actions)
    #
    #   stack: This is the stack at the location where we started
    #          investigating. Which means that the last element of the stack
    #          would be the location where we started.
    #
    #   shift: This is the stack computed after the traversal of edges. It is
    #          reduced each time a reduce action is encountered, and the rest
    #          of the production content is added back to the stack, as newly
    #          acquired context. The last element is the last state reached
    #          through the sequence of lookahead and actions.
    #
    #   lookahead: This is the list of terminals encountered while pushing
    #          edges through the list of terminals.
    #
    #   actions: This is the list of actions that would be executed as we push
    #          edges.

    def next_lookahead_aps(self, aps, st_min, la_min, accept, reject):
        """From a starting state, this function looks from a starting state to reach
        the minimal number of lookahead requested (la_min), while ignoring loops in
        under the minimal number of stack elements of the parser.

        This functions returns 2 lists which are used to resumed the execution
        when one is looking at increasing either the lookahead minimum or the
        stack minimum.

        The first list corresponds to all elements where which have to be
        investigated which are satisfying the requirements of the minimal
        lookahead, and the second list is the list of elements which are not
        interesting because they are including stack state loops.

        """
        st, sh, la, ac = aps
        # TODO: This condition is not suffiscient if there is no more
        # lookahead, in which case we have to fallback on a condition on the
        # stack depth.
        if len(la) >= la_min:
            accept.append(aps)
            return
        if not is_interesting_aps(aps, st_min, la_min):
            reject.append(aps)
            return
        state = self.states[sh[-1]]
        for term, to in state.edges():
            if isinstance(term, Action):
                if isinstance(term, Reduce):
                    for prod in self.reduce_paths[term.nt]:
                        head = self.states[prod[0]]
                        to = head.nonterminals[term.nt]

                        # reduce_paths contains the chains of state shifted to
                        # the parser stack in order to reduce the nonterminal.
                        # When reducing, the stack is resetted to head, and the
                        # nonterminal `term.nt` is pushed, to resume in the
                        # state `to`.
                        if sh[-len(prod):] != prod[-len(sh):]:
                            # If the reduced production does not match the
                            # shifted state, then this reduction does not
                            # apply.
                            continue
                        new_st = prod[:max(len(prod) - len(sh), 0)] + st
                        new_sh = sh[:-len(prod)] + [prod[0], to]
                        new_aps = (new_st, new_sh, la, ac + [term])
                        self.next_lookahead_aps(new_aps, start_aps, accept, reject)
                else:
                    # Actions replace the latest shifted state by a new one.
                    new_aps = (st, sh[:-1] + [to], la, ac + [term])
                    self.next_lookahead_aps(new_aps, start_aps, accept, reject)
            elif not isinstance(term, Nt):
                # terminals are added to the lookahead, and the previous
                # shifted state remains on the emulated parser stack.
                new_aps = (st, sh + [to], la + [term], ac)
                self.next_lookahead_aps(new_aps, start_aps, accept, reject)
        return

    def is_interesting_aps(self, aps, st_min, la_min):
        """We reduced/shifted more than once using the same kind of transitions.
        Do not consider this loop as an interesting case, as it will loop
        indefinitely."""
        st, _, la, _ = aps
        # Reject right recursions when attempting to increase the lookahead.
        if len(st) <= st_min:
            return True
        st = st[:len(st) - st_min]
        if len(st) >= 1 and len(st) != 1 + len(set(zip(st, st[1:]))):
            return False
        return True

    def ambiguous_aps(self, aps1, aps2):
        "Returns True if 2 APS might match each others."
        # NOTE: unused
        st1, _, la1, ac1 = aps1
        st2, _, la2, ac2 = aps2

        if ac1[0] == ac2[0]:
            return False
        if la1[:len(la2)] != la2[:len(la1)]:
            return False
        if st1[-len(st2):] != st2[-len(st1):]:
            return False
        return True

    def is_aps_prefix(self, aps_prefix, aps_query):
        # NOTE: unused
        st1, sh1, la1, _ = aps_prefix
        st2, sh2, la2, _ = aps_query

        if la1 != la2[:len(la1)]:
            return False
        if st1 != st2[-len(st1):]:
            return False
        return True

    def ambiguous_aps_list(self, aps_list):
        # NOTE: This function assumes that the aps_list are already sorted by
        # lookahead and lookbehind patterns.
        act_list = [ a[0] for _, _, _, a in aps_list ]
        if len(set(act_list)) > 1:
            return True
        return False

    def build_lookaround_table(self, s):
        """Given an inconsistent state, which has either a shift-reduce conflict or a
        reduce-reduce conflict. Progressively increase the context necessary to
        disambiguate the inconsistent state.

        This function returns a dictionary which keys are tuple of the sequence
        of testable parser stack, and list of lookahead tokens to be
        considered. The values associated to these keys are the APS (abstract
        parser state) which would be useful to trace the history of the
        modifications back to the original state machine.
        """
        assert self.states[s].is_inconsistent()
        aps = ([s], [s], [], [])
        # This loop progressively increase the context around the inconsistent
        # state, and attempt to build a tree of lookahead and stack context to
        # discriminated between the inconsistencies. When successfully done,
        # the function returns with the decision tree, which would later be
        # converted to a state machine.
        context = 0
        lookaround = {}
        conflict = [aps]
        resolved = []
        while True:
            context += 1
            accepted = []
            rejected = []
            for aps in conflict:
                self.next_lookahead_aps(aps, context, context, accepted, rejected)

            # check what is the minimal set of stack depth that we can check
            # against. NOTE: we increase the lookahead requirement, but this
            # does not garantee that we would have any reduce state to increase
            # the known stack depth.
            st_min = min(context, *[len(st) for st, _, _, _ in accepted ])
            assert st_min >= 1
            lookaround_test = defaultdict(lambda [])
            for aps in accepted:
                st, _, la, _ = aps
                lookaround_test[(tuple(st[:-st_min]), tuple(la))].append(aps)
            ambiguous = False
            accepted = []
            for k, aps_list in lookaround_test:
                if self.ambiguous_aps_list(aps_list):
                    ambiguous = True
                    conflict.extend(aps_list)
                else:
                    lookaround[k] = aps_list
                    resolved.extend(aps_list)

            if not ambiguous:
                return lookaround

            # Insert rules which got rejected as non-interesting for LR(n), and
            # consider them for LR(n+1) as we might accept larger left-right
            # recursion cases due to the extra lookahead.
            conflict.extend(rejected)

    def get_discriminants(lookaround):
        # The best condition is the one which has the minimal entropy.
        #
        # The entropy H of a condition C, which has N successors and A actions
        # possible, is the entropy based on an arbritary "probability" P. P is
        # defined such that, if the condition C successfully discriminate
        # between all the actions, then the entropy should be 0. Otherwise, if
        # the condition C does not hint towards one solution, the entropy
        # should be 1.
        #
        # To fit our expectations, we define the probability P(C = n) as
        # follow: (#A(C = n) - 1) / N * (A - 1) where #A(C = n) is number of
        # actions possible for the successor n. We use (A - 1), such that if
        # there is a unique action, then this condition would have a 0 entropy.
        # We use 1/N as the apriori likelyhood of each condition to be
        # realized.
        #
        # As, the sum of all P(C = n) must be equal to 1, we add an extra fake
        # successor (included in N) which would be composed of all the actions
        # which are not a potential result of the previous conditions.
        #
        #   P(C = N) = 1 - sum(P(C = n) for n in [1..N-1])
        #
        #   H = -sum(P(C = n) log(P(C = n)) for n in [1..N]) / log(N)
        #
        # Examples:
        #
        #   C = 0 -> {A1} and C = 1 -> {A2}, the entropy should be 0.
        #
        #   C = 0 -> {A1, A2} and C = 1 -> {A1, A2}, the entropy should be 1,
        #   if we only have 2 actions possibles.
        #
        #   C = 0 -> {A1, A2} and C = 1 -> {A1}, the entropy should be less
        #   than 1, if we have 2 actions possibles.
        #
        #   C = 0 -> {A1, A2} and C = 1 -> {A1, A2}, the entropy should be less
        #   than 1, if we have more than 2 actions possibles.
        hits = defaultdict(lambda defaultdict(lambda set()))
        common_depth = min([len(k) for k, _ in lookaround ])
        num_A = len(set([ aps_list[0][3][0] for _, aps_list in lookaround ]))
        for k, aps_list in lookaround:
            behind, ahead = k
            action = aps_list[0][3][0]
            hits[1][ahead[0]].append(action)
            behind = behind[len(behind) - common_depth:]
            for i, s in enumerate(behind, 1 - common_depth):
                hits[i][s].append(action)

        # Compute entropy of each condition.
        entropy = {}
        for i, key_actions in hits:
            entropy[i] = 0.0
            rest = 1.0
            num_N = len(key_actions) + 1.0
            for k, actions in key_actions:
                prob = (len(actions) - 1.0) / (num_N * (num_A - 1.0))
                assert 0.0 <= prob and prob <= 1.0
                if prob > 0.0:
                    entropy[i] += - prob * math.log(prob)
                rest -= prob
            assert 0.0 <= rest and rest <= 1.0
            if rest > 0.0:
                entropy[i] += - rest * math.log(rest)
            entropy[i] = entropy[i] / math.log(num_N)

        min_entropy = min([ e for i, e in entropy ])
        selected = [ i for i, e in entropy if e == min_entropy ][0]


    def fix_inconsistent_state(self, s, lookaround):
        """We manage to build a non-ambiguous lookaround table for the inconsistent
        state. Our goal is now to convert the lookaround table in a decision tree."""

        # Ideally we would want to discriminate the cases based on the
        # frequencies of which they are used, and order the discriminant based
        # on their ability to split the group the fastest. However, we can
        # always check for look-behind cases, and only check for the first
        # terminal expected.
        #
        # NOTE: in case of right-recursive grammars, we have to add a push &
        # pop flag to filter the origin. The original idea is to split the
        # graph based on the origin, which increases the parse table size. In
        # the worse case, adding a push action would result in the duplication
        # of all the states, but this remains unlikely.
        #
        # For left-recursive grammars, we can add a context check which look-up
        # within the stack of the parser in which state we are, and how to
        # discriminate.
        #
        # For shift-reduce issues, we can add more look-ahead tokens to figure
        # out which statement we are reducing.
        return

    def fix_inconsistent_table(self, verbose, progress):
        """The parse table might be inconsistent. We fix the parse table by looking
        around the inconsistent states for more context. Either by looking at the
        potential stack state which might lead to the inconsistent state, or by
        increasing the lookahead."""
        if verbose:
            print("Fix parse table incosistencies.")

        todo = collections.deque()
        for s in self.goals:
            todo.append([s])

        def visit_table():
            while todo:
                yield # progress bar.
                # TODO: Compare stack / queue, for the traversal of the states.
                shifted = todo.popleft()
                state = self.states[shifted[-1]]
                if state.is_inconsistent():
                    lookaround = self.build_lookaround_table()
                    self.fix_inconsistent_state(s, lookaround)

        consume(visit_table(), progress)


def generate_parser_states(grammar, *, verbose=False, progress=False):
    assert isinstance(grammar, Grammar)
    grammar = CanonicalGrammar(grammar)

    # Now the grammar is in its final form. Compute information about it that
    # we can cache and use during the main part of the algorithm below.
    start = start_sets(grammar.grammar)
    start_set_cache = make_start_set_cache(grammar.grammar, grammar.prods, start)
    follow = follow_sets(grammar.grammar, grammar.prods_with_indexes_by_nt, start_set_cache)
    context = PgenContext(
        grammar.grammar, grammar.prods, grammar.prods_with_indexes_by_nt, start, start_set_cache, follow)

    # Run the core LR table generation algorithm.
    return analyze_states(context, grammar.prods, verbose=verbose, progress=progress)


def generate_parser(out, source, *, verbose=False, progress=False, target='python', handler_info=None):
    assert target in ('python', 'rust')

    if isinstance(source, Grammar):
        parser_states = generate_parser_states(
            source, verbose=verbose, progress=progress)
    elif isinstance(source, ParserStates):
        parser_states = source
    else:
        raise TypeError("unrecognized source: {!r}".format(source))

    if target == 'rust':
        emit.write_rust_parser(out, parser_states, handler_info)
    else:
        emit.write_python_parser(out, parser_states)


def compile(grammar):
    assert isinstance(grammar, Grammar)
    out = io.StringIO()
    generate_parser(out, grammar)
    scope = {}
    # print(out.getvalue())
    exec(out.getvalue(), scope)
    return scope['Parser']


# *** Fun demo ****************************************************************

def demo():
    from .grammar import example_grammar
    grammar = example_grammar()

    from . import lexer
    tokenize = lexer.LexicalGrammar(
        "+ - * / ( )", NUM=r'0|[1-9][0-9]*', VAR=r'[_A-Za-z]\w+')

    import io
    out = io.StringIO()
    generate_parser(out, grammar)
    code = out.getvalue()
    print(code)
    print("----")

    sandbox = {}
    exec(code, sandbox)
    parse = sandbox['parse_expr']

    while True:
        try:
            line = input('> ')
        except EOFError:
            break

        try:
            result = parse(tokenize, line)
        except Exception as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
        else:
            print(result)


if __name__ == '__main__':
    demo()
