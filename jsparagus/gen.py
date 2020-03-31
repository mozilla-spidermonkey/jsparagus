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

from __future__ import annotations

import collections
import dataclasses
import io
import pickle
import sys
import hashlib
import typing
from dataclasses import dataclass

from .ordered import OrderedSet, OrderedFrozenSet

from .grammar import (Grammar,
                      NtDef, Production, Some, CallMethod, InitNt,
                      ReduceExpr, ReduceExprOrAccept,
                      Element, is_concrete_element,
                      Optional, Exclude, Literal, UnicodeCategory, Nt, Var,
                      End, NoLineTerminatorHere, ErrorSymbol,
                      LookaheadRule, LenientNt)
from .actions import (Action, Reduce, Lookahead, CheckNotOnNewLine, FilterFlag,
                      FunCall, Seq)
from . import emit
from .runtime import ErrorToken, ErrorTokenClass
from .utils import keep_until
from . import types
from .aps import Edge, APS


# *** Operations on grammars **************************************************

T = typing.TypeVar("T")


def fix(f: typing.Callable[[T], T], start: T) -> T:
    """Compute a fixed point of `f`, the hard way, starting from `start`."""
    prev, current = start, f(start)
    while current != prev:
        prev, current = current, f(current)
    return current


def empty_nt_set(grammar: Grammar) -> typing.Dict[LenientNt, ReduceExprOrAccept]:
    """Determine which nonterminals in `grammar` can produce the empty string.

    Return a dict {nt: expr} that maps each such nonterminal to the expr
    that should be evaluated when reducing the empty string to nt.
    So, for example, if we have a production

        a ::= b? c?  => CallMethod("a", [0, 1])

    then the resulting dictionary will contain the entry
    `("a", CallMethod("a", [None, None]))`.
    """

    empties: typing.Dict[LenientNt, ReduceExprOrAccept] = {}

    def production_is_empty(p: Production) -> bool:
        return all(isinstance(e, LookaheadRule)
                   or isinstance(e, Optional)
                   or (isinstance(e, Nt) and e in empties)
                   or e is NoLineTerminatorHere
                   for e in p.body)

    def evaluate_reducer_with_empty_matches(p: Production) -> ReduceExprOrAccept:
        # partial evaluation of p.reducer
        stack = [e for e in p.body if is_concrete_element(e)]

        Expr = typing.TypeVar("Expr", ReduceExpr, ReduceExprOrAccept)

        def eval(expr: Expr) -> Expr:
            if expr is None:
                return None
            elif isinstance(expr, Some):
                return Some(eval(expr.inner))
            elif isinstance(expr, CallMethod):
                return dataclasses.replace(
                    expr,
                    args=tuple(eval(arg_expr) for arg_expr in expr.args)
                )
            elif isinstance(expr, int):
                e = stack[expr]
                if isinstance(e, Optional):
                    return None
                else:
                    assert isinstance(e, Nt)
                    result = empties[e]
                    assert not isinstance(result, str)
                    return result
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
                    if production_is_empty(p):
                        if nt in empties:
                            raise ValueError(
                                "ambiguous grammar: multiple productions for "
                                "{!r} match the empty string"
                                .format(nt))
                        done = False
                        empties[nt] = evaluate_reducer_with_empty_matches(p)
    return empties


def check_cycle_free(grammar: Grammar) -> None:
    """Throw an exception if any nonterminal in `grammar` produces itself
    via a cycle of 1 or more productions.
    """
    empties = empty_nt_set(grammar)

    # OK, first find out which nonterminals directly produce which other
    # nonterminals (after possibly erasing some optional/empty nts).
    direct_produces: typing.Dict[LenientNt, typing.Set[Nt]] = {}
    for orig in grammar.nonterminals:
        direct_produces[orig] = set()
        for source_production in grammar.nonterminals[orig].rhs_list:
            for rhs, _r in expand_optional_symbols_in_rhs(source_production.body, grammar, empties):
                result: typing.List[Nt] = []
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
                    elif isinstance(e, Exclude):
                        if isinstance(e.inner, Nt):
                            result.append(e.inner)
                    elif isinstance(e, LookaheadRule):
                        # Ignore the restriction. We lose a little precision
                        # here, and could report a cycle where there isn't one,
                        # but it's unlikely in real-world grammars.
                        pass
                    elif e is NoLineTerminatorHere:
                        # This doesn't affect the property we're checking.
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
                    elif isinstance(e, End):
                        # This production consume the End meta-character,
                        # therefore it cannot correspond to an empty cycle,
                        # even if this character is expect to be produced
                        # infinitely once the end is reached.
                        break
                    elif isinstance(e, CallMethod):
                        # This production execute code, but does not consume
                        # any input.
                        pass
                    else:
                        # Optional is not possible because we called
                        # expand_optional_symbols_in_rhs. ErrorSymbol
                        # effectively matches the empty string (though only if
                        # nothing else matches).
                        assert isinstance(e, ErrorSymbol)
                else:
                    # If we get here, we didn't break, so our results are good!
                    # nt can definitely produce all the nonterminals in result.
                    direct_produces[orig] |= set(result)

    def step(
            produces: typing.Dict[LenientNt, typing.Set[Nt]]
    ) -> typing.Dict[LenientNt, typing.Set[Nt]]:
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


def check_lookahead_rules(grammar: Grammar) -> None:
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


def check_no_line_terminator_here(grammar: Grammar) -> None:
    empties = empty_nt_set(grammar)

    def check(e: Element, nt: LenientNt, body: typing.List[Element]) -> None:
        if grammar.is_terminal(e):
            pass
        elif isinstance(e, Nt):
            if e in empties:
                raise ValueError(
                    "invalid grammar: [no LineTerminator here] cannot appear next to "
                    "a nonterminal that matches the empty string\n"
                    "in production: {}".format(grammar.production_to_str(nt, body)))
        else:
            raise ValueError(
                "invalid grammar: [no LineTerminator here] must appear only "
                "between terminals and/or nonterminals\n"
                "in production: {}".format(grammar.production_to_str(nt, body)))

    for nt in grammar.nonterminals:
        for production in grammar.nonterminals[nt].rhs_list:
            body = production.body
            for i, e in enumerate(body):
                if e is NoLineTerminatorHere:
                    if i == 0 or i == len(body) - 1:
                        raise ValueError(
                            "invalid grammar: [no LineTerminator here] must be between two symbols\n"
                            "in production: {}".format(grammar.production_to_str(nt, body)))
                    check(body[i - 1], nt, body)
                    check(body[i + 1], nt, body)


def expand_parameterized_nonterminals(grammar: Grammar):
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

    def expand(nt: Nt) -> NtDef:
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

        def expand_element(e: Element) -> Element:
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

        def expand_production(p: Production) -> Production:
            return p.copy_with(
                body=[expand_element(e) for e in p.body],
                condition=None)

        def expand_productions(nt_def: NtDef) -> NtDef:
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

    # "type: ignore" because this runs afoul of Python's covariance rules for
    # Mapping; but it conforms to the intended type.
    return grammar.with_nonterminals(new_nonterminals)  # type: ignore


# *** Start sets and follow sets **********************************************

EMPTY = "(empty)"
END = None

TerminalOrEmpty = str
TerminalOrEmptyOrErrorToken = typing.Union[str, ErrorTokenClass]
StartSets = typing.Dict[Nt, OrderedFrozenSet[TerminalOrEmpty]]


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


def seq_start(
        grammar: Grammar,
        start: StartSets,
        seq: typing.List[Element]
) -> OrderedFrozenSet[TerminalOrEmptyOrErrorToken]:
    """Compute the start set for a sequence of elements."""
    s: OrderedSet[TerminalOrEmptyOrErrorToken] = OrderedSet([EMPTY])
    for i, e in enumerate(seq):
        if EMPTY not in s:  # preceding elements never match the empty string
            break
        s.remove(EMPTY)
        if grammar.is_terminal(e):
            assert isinstance(e, str)
            s.add(e)
        elif isinstance(e, ErrorSymbol):
            s.add(ErrorToken)
        elif isinstance(e, Nt):
            s |= start[e]
        elif e is NoLineTerminatorHere:
            s.add(EMPTY)
        else:
            assert isinstance(e, LookaheadRule)
            future = seq_start(grammar, start, seq[i + 1:])
            if e.positive:
                future &= e.set
            else:
                future -= e.set
            return OrderedFrozenSet(future)
    return OrderedFrozenSet(s)


StartSetCache = typing.List[typing.List[OrderedFrozenSet[TerminalOrEmptyOrErrorToken]]]


def make_start_set_cache(
        grammar: Grammar,
        prods: typing.List[Prod],
        start: StartSets
) -> StartSetCache:
    """Compute start sets for all suffixes of productions in the grammar.

    Returns a list of lists `cache` such that
    `cache[n][i] == seq_start(grammar, start, prods[n][i:])`.

    (The cache is for speed, since seq_start was being called millions of
    times.)
    """

    def suffix_start_list(
            rhs: typing.List[Element]
    ) -> typing.List[OrderedFrozenSet[TerminalOrEmptyOrErrorToken]]:
        sets: typing.List[OrderedFrozenSet[TerminalOrEmptyOrErrorToken]]
        sets = [OrderedFrozenSet([EMPTY])]
        for e in reversed(rhs):
            s: OrderedFrozenSet[TerminalOrEmptyOrErrorToken]
            if grammar.is_terminal(e):
                assert isinstance(e, str)
                s = OrderedFrozenSet([e])
            elif isinstance(e, ErrorSymbol):
                s = OrderedFrozenSet([ErrorToken])
            elif isinstance(e, Nt):
                s = start[e]
                if EMPTY in s:
                    s = OrderedFrozenSet((s - {EMPTY}) | sets[-1])
            elif e is NoLineTerminatorHere:
                s = sets[-1]
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
@dataclass
class Prod:
    nt: LenientNt
    index: int
    rhs: typing.List
    reducer: ReduceExprOrAccept


def expand_optional_symbols_in_rhs(
        rhs: typing.List[Element],
        grammar: Grammar,
        empties: typing.Dict[LenientNt, ReduceExprOrAccept],
        start_index: int = 0
) -> typing.Iterable[typing.Tuple[typing.List[Element], typing.Dict[int, ReduceExpr]]]:
    """Expand a sequence with optional symbols into sequences that have none.

    rhs is a list of symbols, possibly containing optional elements. This
    yields every list that can be made by replacing each optional element
    either with its .inner value, or with nothing.

    Each list is accompanied by the list of the indices of optional elements in
    `rhs` that were dropped.

    For example, `expand_optional_symbols_in_rhs(["if", Optional("else")])`
    yields the two pairs `(["if"], [1])` and `["if", "else"], []`.
    """

    replacement: ReduceExpr
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
            empty_expr = empties[e]
            # The replacement can't be 'accept' because that only happens with
            # InitNt nonterminals, which are never used in productions.
            assert not isinstance(empty_expr, str)
            replacement = empty_expr
            break
    else:
        yield rhs[start_index:], {}
        return

    for expanded, r in expand_optional_symbols_in_rhs(rhs, grammar, empties, i + 1):
        e = rhs[i]
        rhs_inner = e.inner if isinstance(e, Optional) else e
        # without rhs[i]
        r2 = r.copy()
        r2[i] = replacement
        yield rhs[start_index:i] + expanded, r2
        # with rhs[i]
        yield rhs[start_index:i] + [rhs_inner] + expanded, r


def expand_all_optional_elements(grammar: Grammar) -> typing.Tuple[
        Grammar,
        typing.List[Prod],
        typing.DefaultDict[LenientNt, typing.List[typing.Tuple[int, typing.List[Element]]]]
]:
    """Expand optional elements in the grammar.

    We replace each production that contains an optional element with two
    productions: one with and one without. Downstream of this step, we can
    ignore the possibility of optional elements.
    """
    expanded_grammar: typing.Dict[LenientNt, NtDef] = {}

    # This was capturing the set of empty production to simplify the work of
    # the previous algorithm which was trying to determine the lookahead.
    # However, with the LR0Generator this is no longer needed as we are
    # generating deliberatly inconsistent parse table states, which are then
    # properly fixed by adding lookahead information where needed, and without
    # bugs!
    # empties = empty_nt_set(grammar)
    empties: typing.Dict[LenientNt, ReduceExprOrAccept] = {}

    # Put all the productions in one big list, so each one has an index. We
    # will use the indices in the action table (as reduce action payloads).
    prods: typing.List[Prod] = []
    prods_with_indexes_by_nt: \
        typing.DefaultDict[LenientNt, typing.List[typing.Tuple[int, typing.List[Element]]]] = \
        collections.defaultdict(list)

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

                Expr = typing.TypeVar("Expr", ReduceExpr, ReduceExprOrAccept)

                def adjust_reduce_expr(expr: Expr) -> Expr:
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
                        return dataclasses.replace(
                            expr,
                            args=tuple(adjust_reduce_expr(arg)
                                       for arg in expr.args)
                        )
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


@dataclass(frozen=True, order=True)
class LRItem:
    """A snapshot of progress through a single specific production.

    *   `prod_index` identifies the production. (Every production in the grammar
        gets a unique index; see the loop that computes
        prods_with_indexes_by_nt.)

    *   `offset` is the position of the cursor within the production.

    `lookahead` and `followed_by` are two totally different kinds of lookahead.

    *   `lookahead` is the LookaheadRule, if any, that applies to the immediately
        upcoming input. It is present only if this LRItem is subject to a
        `[lookahead]` restriction; otherwise it's None. These restrictions can't
        extend beyond the end of a production, or else the grammar is invalid.
        This implements the lookahead restrictions in the ECMAScript grammar.
        It is not part of any account of LR I've seen.

    *   `followed_by` is a completely different kind of lookahead restriction.
        This is the kind of lookahead that is a central part of canonical LR
        table generation.  It applies to the token *after* the whole current
        production, so `followed_by` always applies to completely different and
        later tokens than `lookahead`.  `followed_by` is a set of terminals; if
        `None` is in this set, it means `END`, not that the LRItem is
        unrestricted.
    """

    prod_index: int
    offset: int
    lookahead: typing.Optional[LookaheadRule]
    followed_by: OrderedFrozenSet[typing.Optional[str]]


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


class CanonicalGrammar:
    __slots__ = "prods", "prods_with_indexes_by_nt", "grammar"

    def __init__(self, grammar, old=False):
        assert isinstance(grammar, Grammar)

        # Step by step, we check the grammar and lower it to a more primitive form.
        grammar = expand_parameterized_nonterminals(grammar)
        check_cycle_free(grammar)
        # check_lookahead_rules(grammar)
        check_no_line_terminator_here(grammar)
        grammar, prods, prods_with_indexes_by_nt = expand_all_optional_elements(
            grammar)
        self.prods = prods
        self.prods_with_indexes_by_nt = prods_with_indexes_by_nt
        self.grammar = grammar


class StateAndTransitions:
    """This is one state of the parse table, which has transitions based on
    terminals (text), non-terminals (grammar rules) and epsilon (reduce).

    In this model epsilon transitions are used to represent code to be executed
    such as reduce actions and any others actions.
    """

    __slots__ = [
        "index",            # Numerical index of this state.
        "terminals",        # Terminals map.
        "nonterminals",     # Non-terminals map.
        "errors",           # List of error symbol transitions.
        "epsilon",          # List of epsilon transitions with associated
                            # actions.
        "locations",        # Ordered set of LRItems (grammar position).
        "delayed_actions",  # Ordered set of Actions which are pushed to the
                            # next state after a conflict.
        "backedges",        # Set of back edges and whether these are expected
                            # to be on the parser stack or not.
        "_hash",            # Hash to identify states which have to be merged.
                            # This hash is composed of LRItems of the LR0
                            # grammar, and actions performed on it since. This
                            # hash is sued for python internals such as
                            # dictionary and sets.
        "stable_hash",      # Hash computed the same way as the previous one,
                            # but used only for human readbale output. The
                            # stability is useful to match states across
                            # grammar modifications.
    ]

    def __init__(self, index, locations, delayed_actions=OrderedFrozenSet()):
        assert isinstance(locations, OrderedFrozenSet)
        assert isinstance(delayed_actions, OrderedFrozenSet)
        self.index = index
        self.terminals = {}
        self.nonterminals = {}
        self.errors = {}
        self.epsilon = []
        self.locations = locations
        self.delayed_actions = delayed_actions
        self.backedges = OrderedSet()

        # NOTE: The hash of a state depends on its location in the LR0
        # parse-table, as well as the actions which have not yet been executed.
        def hashed_content():
            for item in sorted(self.locations):
                yield item
                yield "\n"
            yield "delayed_actions"
            for action in self.delayed_actions:
                yield hash(action)

        self._hash = hash(tuple(hashed_content()))
        h = hashlib.md5()
        h.update("".join(map(str, hashed_content())).encode())
        self.stable_hash = h.hexdigest()[:6]

    def is_inconsistent(self):
        "Returns True if the state transitions are inconsistent."
        # TODO: We could easily allow having a state with non-terminal
        # transition and other epsilon transitions, as the non-terminal shift
        # transitions are a form of condition based on the fact that a
        # non-terminal, produced by a reduce action is consumed by the
        # automaton.
        if len(self.terminals) + len(self.nonterminals) + len(self.errors) > 0 and len(self.epsilon) > 0:
            return True
        elif len(self.epsilon) == 1:
            if any(k.is_inconsistent() for k, s in self.epsilon):
                return True
        elif len(self.epsilon) > 1:
            if any(k.is_inconsistent() for k, s in self.epsilon):
                return True
            # If all the out-going edges are FilterFlags, with the same flag
            # and different values, then this state remains consistent, as this
            # can be implemented as a deterministic switch statement.
            if any(not k.is_condition() for k, s in self.epsilon):
                return True
            if any(not isinstance(k.condition(), FilterFlag) for k, s in self.epsilon):
                return True
            if len(set(k.condition().flag for k, s in self.epsilon)) > 1:
                return True
            if len(self.epsilon) != len(set(k.condition().value for k, s in self.epsilon)):
                return True
        else:
            try:
                self.get_error_symbol()
            except ValueError:
                return True
        return False

    def shifted_edges(self):
        for k, s in self.terminals.items():
            yield (k, s)
        for k, s in self.nonterminals.items():
            yield (k, s)
        for k, s in self.errors.items():
            yield (k, s)

    def edges(self):
        for k, s in self.terminals.items():
            yield (k, s)
        for k, s in self.nonterminals.items():
            yield (k, s)
        for k, s in self.errors.items():
            yield (k, s)
        for k, s in self.epsilon:
            yield (k, s)

    def rewrite_state_indexes(self, state_map):
        self.index = state_map[self.index]
        self.terminals = {
            k: state_map[s] for k, s in self.terminals.items()
        }
        self.nonterminals = {
            k: state_map[s] for k, s in self.nonterminals.items()
        }
        self.errors = {
            k: state_map[s] for k, s in self.errors.items()
        }
        self.epsilon = [
            (k, state_map[s]) for k, s in self.epsilon
        ]
        self.backedges = set(
            Edge(state_map[edge.src], edge.term) for edge in self.backedges
        )

    def get_error_symbol(self):
        if len(self.errors) > 1:
            raise ValueError("More than one error symbol on the same state.")
        else:
            return next(iter(self.errors), None)

    def __contains__(self, term):
        if isinstance(term, Action):
            for t, s in self.epsilon:
                if t == term:
                    return True
            return False
        elif isinstance(term, Nt):
            return term in self.nonterminals
        elif isinstance(term, ErrorSymbol):
            return term in self.errors
        else:
            return term in self.terminals

    def __getitem__(self, term):
        if isinstance(term, Action):
            for t, s in self.epsilon:
                if t == term:
                    return s
            raise KeyError(term)
        elif isinstance(term, Nt):
            return self.nonterminals[term]
        if isinstance(term, ErrorSymbol):
            return self.errors[term]
        else:
            return self.terminals[term]

    def get(self, term, default):
        try:
            return self.__getitem__(term)
        except KeyError:
            return default

    def stable_str(self, states):
        conflict = ""
        if self.is_inconsistent():
            conflict = " (inconsistent)"
        return "{}{}:\n{}".format(self.stable_hash, conflict, "\n".join([
            "\t{} --> {}".format(k, states[s].stable_hash) for k, s in self.edges()]))

    def __str__(self):
        conflict = ""
        if self.is_inconsistent():
            conflict = " (inconsistent)"
        return "{}{}:\n{}".format(self.index, conflict, "\n".join([
            "\t{} --> {}".format(k, s) for k, s in self.edges()]))

    def __eq__(self, other):
        assert isinstance(other, StateAndTransitions)
        if sorted(self.locations) != sorted(other.locations):
            return False
        if sorted(self.delayed_actions) != sorted(other.delayed_actions):
            return False
        return True

    def __hash__(self):
        return self._hash


def on_stack(grammar, term):
    """Returns whether an element of a production is consuming stack space or
    not."""
    if isinstance(term, Nt):
        return True
    elif grammar.is_terminal(term):
        return True
    elif isinstance(term, LookaheadRule):
        return False
    elif isinstance(term, ErrorSymbol):
        return True
    elif isinstance(term, End):
        return True
    elif term is NoLineTerminatorHere:
        # No line terminator is a property of the next token being shifted. It
        # is implemented as an action which once shifted past the next term,
        # will check whether the previous term shifted is on a new line.
        return False
    elif isinstance(term, CallMethod):
        return False
    raise ValueError(term)


def callmethods_to_funcalls(expr, pop, ret, depth, funcalls):
    # TODO: find a way to carry alias sets to here.
    alias_set = ["parser"]
    if isinstance(expr, int):
        stack_index = pop - expr
        if depth == 0:
            expr = FunCall("id", (stack_index,), fallible=False,
                           trait=types.Type("AstBuilder"), set_to=ret,
                           alias_read=alias_set, alias_write=alias_set)
            funcalls.append(expr)
            return ret
        else:
            return stack_index
    elif isinstance(expr, Some):
        res = callmethods_to_funcalls(expr.inner, pop, ret, depth, funcalls)
        return Some(res)
    elif expr is None:
        return None
    elif isinstance(expr, CallMethod):
        def convert_args(args):
            for i, arg in enumerate(args):
                yield callmethods_to_funcalls(arg, pop, ret + "_{}".format(i), depth + 1, funcalls)
        args = tuple(convert_args(expr.args))
        expr = FunCall(expr.method, args,
                       trait=expr.trait,
                       fallible=expr.fallible,
                       set_to=ret,
                       alias_read=alias_set,
                       alias_write=alias_set)
        funcalls.append(expr)
        return ret
    elif expr == "accept":
        expr = FunCall("accept", (),
                       trait=types.Type("ParserTrait"),
                       fallible=False,
                       set_to=ret,
                       alias_read=alias_set,
                       alias_write=alias_set)
        funcalls.append(expr)
        return ret
    else:
        raise ValueError(expr)


class LR0Generator:
    """Provide a way to iterate over the grammar, given a set of LR items."""
    __slots__ = [
        "grammar",
        "lr_items",
        "key",
        "_hash",
    ]

    grammar: CanonicalGrammar
    lr_items: OrderedFrozenSet[LRItem]
    key: str
    _hash: int

    def __init__(
            self,
            grammar: CanonicalGrammar,
            lr_items: typing.Iterable[LRItem] = ()
    ) -> None:
        self.grammar = grammar
        self.lr_items = OrderedFrozenSet(lr_items)
        # This is used to reuse states which have already been encoded.
        self.key = "".join(repr((item.prod_index, item.offset)) + "\n"
                           for item in sorted(self.lr_items))
        self._hash = hash(self.key)

    def __eq__(self, other: object) -> bool:
        return isinstance(other, LR0Generator) and self.key == other.key

    def __hash__(self) -> int:
        return self._hash

    def __str__(self) -> str:
        s = ""
        for lr_item in self.lr_items:
            s += self.grammar.grammar.lr_item_to_str(self.grammar.prods, lr_item)
            s += "\n"
        return s

    def stable_locations(self):
        locations = []
        for lr_item in self.lr_items:
            locations.append(self.grammar.grammar.lr_item_to_str(self.grammar.prods, lr_item))
        return OrderedFrozenSet(sorted(locations))

    @staticmethod
    def start(grammar: CanonicalGrammar, nt: Nt) -> LR0Generator:
        lr_items: typing.List[LRItem] = []
        # Visit the initial non-terminal, as well as all the non-terminals
        # which are at the left of each productions.
        todo: typing.Deque[Nt] = collections.deque()
        visited_nts = []
        todo.append(nt)
        while todo:
            nt = todo.popleft()
            if nt in visited_nts:
                continue
            visited_nts.append(nt)
            for prod_index, _ in grammar.prods_with_indexes_by_nt[nt]:
                assert isinstance(prod_index, int)
                lr_items.append(LRItem(
                    prod_index=prod_index,
                    offset=0,
                    lookahead=None,
                    followed_by=OrderedFrozenSet(),
                ))

                prod = grammar.prods[prod_index]
                assert isinstance(prod, Prod)
                try:
                    term = prod.rhs[0]
                    if isinstance(term, Nt):
                        todo.append(term)
                except IndexError:
                    pass
        return LR0Generator(grammar, lr_items)

    def transitions(self):
        """Returns the dictionary which maps the state transitions with the next
        LR0Generators. This can be used to generate the states and the
        transitions between the states of an LR0 parse table."""
        followed_by = collections.defaultdict(lambda: [])
        for lr_item in self.lr_items:
            self.item_transitions(lr_item, followed_by)

        for k, lr_items in followed_by.items():
            followed_by[k] = LR0Generator(self.grammar, lr_items)

        return followed_by

    def item_transitions(self, lr_item, followed_by):
        """Given one LR Item, register all the transitions and LR Items reachables
        through these transitions."""
        prod = self.grammar.prods[lr_item.prod_index]
        assert isinstance(prod, Prod)

        # Read the term located at the offset in the production.
        if lr_item.offset < len(prod.rhs):
            term = prod.rhs[lr_item.offset]
            if isinstance(term, Nt):
                pass
            elif self.grammar.grammar.is_terminal(term):
                pass
            elif isinstance(term, LookaheadRule):
                term = Lookahead(term.set, term.positive)
            elif isinstance(term, ErrorSymbol):
                # ErrorSymbol as considered as terminals. These terminals would
                # be produced by the error handling code which produces these
                # error symbols on-demand.
                pass
            elif isinstance(term, End):
                # End is considered as a terminal which is produduced once by
                # the lexer upon reaching the end. However, the parser might
                # finish without consuming the End terminal, if there is no
                # ambiguity on whether the End is expected.
                pass
            elif term is NoLineTerminatorHere:
                # Check whether the following terminal is on a new line. If
                # not, this would produce a syntax error. The argument is the
                # terminal offset.
                term = CheckNotOnNewLine()
            elif isinstance(term, CallMethod):
                funcalls = []
                pop = sum(1 for e in prod.rhs[:lr_item.offset] if on_stack(self.grammar.grammar, e))
                callmethods_to_funcalls(term, pop, "expr", 0, funcalls)
                term = Seq(funcalls)

        elif lr_item.offset == len(prod.rhs):
            # Add the reduce operation as a state transition in the generated
            # parse table. (TODO: this supposed that the canonical form did not
            # move the reduce action to be part of the production)
            pop = sum(1 for e in prod.rhs if on_stack(self.grammar.grammar, e))
            term = Reduce(prod.nt, pop)
            expr = prod.reducer
            if expr is not None:
                funcalls = []
                callmethods_to_funcalls(expr, pop, "value", 0, funcalls)
                term = Seq(funcalls + [term])
        else:
            # No edges after the reduce operation.
            return

        # Add terminals, non-terminals and lookahead actions, as transitions to
        # the next LR Item.
        new_transition = term not in followed_by
        followed_by[term].append(LRItem(
            prod_index=lr_item.prod_index,
            offset=lr_item.offset + 1,
            lookahead=None,
            followed_by=OrderedFrozenSet(),
        ))

        # If the term is a non-terminal, then recursively add transitions from
        # the beginning of all the productions which are matching this
        # non-terminal.
        #
        # Only do it once per non-terminal to avoid infinite recursion on
        # left-recursive grammars.
        if isinstance(term, Nt) and new_transition:
            for prod_index, _ in self.grammar.prods_with_indexes_by_nt[term]:
                assert isinstance(prod_index, int)
                self.item_transitions(LRItem(
                    prod_index=prod_index,
                    offset=0,
                    lookahead=None,
                    followed_by=OrderedFrozenSet(),
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
        # Map of state object to the corresponding identifer.
        "state_cache",
        # List of (Nt, states) tuples which are the entry point of the state
        # machine.
        "named_goals",
        # List of terminals.
        "terminals",
        # List of non-terminals.
        "nonterminals",
        # Set of existing flags.
        "flags",
        # Carry the info to be used when generating debug_context. If False,
        # then no debug_context is ever produced.
        "debug_info",
        # Execution modes are used by the code generator to decide which
        # function is executed when. This is a dictionary of OrderedSet, where
        # the keys are the various parsing modes, and the mapped set contains
        # the list of traits which have to be implemented, and consequently
        # which functions would be encoded.
        "exec_modes",
        # Boolean flag which assume whether or not the parse table is
        # inconsistent. This is used for adding extra assertion when computing
        # the reduce path.
        "assume_inconsistent",
    ]

    def __init__(self, grammar, verbose=False, progress=False, debug=False):
        self.actions = []
        self.states = []
        self.state_cache = {}
        self.named_goals = []
        self.terminals = grammar.grammar.terminals
        self.nonterminals = list(grammar.grammar.nonterminals.keys())
        self.debug_info = debug
        self.exec_modes = grammar.grammar.exec_modes
        self.assume_inconsistent = True
        self.create_lr0_table(grammar, verbose, progress)
        self.fix_inconsistent_table(verbose, progress)
        # TODO: Optimize chains of actions into sequences.
        # Optimize by removing unused states.
        self.remove_all_unreachable_state(verbose, progress)
        # TODO: Statically compute replayed terms. (maybe?)
        # Fold paths which have the same ending.
        self.fold_identical_endings(verbose, progress)
        # Split shift states from epsilon states.
        self.group_epsilon_states(verbose, progress)

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

    def is_inconsistent(self):
        "Returns True if the grammar contains any inconsistent state."
        for s in self.states:
            if s is not None and s.is_inconsistent():
                return True
        return False

    def new_state(self, locations, delayed_actions=OrderedFrozenSet()):
        """Get or create state with an LR0 location and delayed actions. Returns a tuple
        where the first element is whether the element is newly created, and
        the second element is the State object."""
        index = len(self.states)
        state = StateAndTransitions(index, locations, delayed_actions)
        try:
            return False, self.state_cache[state]
        except KeyError:
            self.state_cache[state] = state
            self.states.append(state)
            return True, state

    def get_state(self, locations, delayed_actions=OrderedFrozenSet()):
        """Like new_state(), but only returns the state without returning whether it is
        newly created or not."""
        _, state = self.new_state(locations, delayed_actions)
        return state

    def remove_state(self, s, maybe_unreachable_set):
        state = self.states[s]
        # print("Remove state {}".format(state))
        self.clear_edges(state, maybe_unreachable_set)
        del self.state_cache[state]
        self.states[s] = None

    def add_edge(self, src, term, dest):
        assert isinstance(src, StateAndTransitions)
        assert term not in src
        assert isinstance(dest, int) and dest < len(self.states)
        if isinstance(term, Action):
            src.epsilon.append((term, dest))
        elif isinstance(term, Nt):
            src.nonterminals[term] = dest
        elif isinstance(term, ErrorSymbol):
            src.errors[term] = dest
        else:
            src.terminals[term] = dest
        self.states[dest].backedges.add(Edge(src.index, term))

    def remove_backedge(self, src, term, dest, maybe_unreachable_set):
        self.states[dest].backedges.remove(Edge(src.index, term))
        maybe_unreachable_set.add(dest)

    def replace_edge(self, src, term, dest, maybe_unreachable_set):
        assert isinstance(src, StateAndTransitions)
        assert isinstance(dest, int) and dest < len(self.states)

        if term in src:
            old_dest = src[term]
            self.remove_backedge(src, term, old_dest, maybe_unreachable_set)

        if isinstance(term, Action):
            src.epsilon = [(t, d) for t, d in src.epsilon if t != term]
            src.epsilon.append((term, dest))
        elif isinstance(term, Nt):
            src.nonterminals[term] = dest
        elif isinstance(term, ErrorSymbol):
            src.errors[term] = dest
        else:
            src.terminals[term] = dest
        self.states[dest].backedges.add(Edge(src.index, term))

    def clear_edges(self, src, maybe_unreachable_set):
        """Remove all existing edges, in order to replace them by new one. This is used
        when resolving shift-reduce conflicts."""
        assert isinstance(src, StateAndTransitions)
        for term, dest in src.edges():
            self.remove_backedge(src, term, dest, maybe_unreachable_set)
        src.terminals = {}
        src.nonterminals = {}
        src.errors = {}
        src.epsilon = []

    def remove_unreachable_states(self, maybe_unreachable_set):
        # TODO: This function is incomplete in case of loops, some cycle might
        # remain isolated while not being reachable from the init states. We
        # should maintain a notion of depth per-state, such that we can
        # identify loops by noticing the all backedges have a larger depth than
        # the current state.
        _, init = zip(*self.named_goals)
        init = set(init)
        while maybe_unreachable_set:
            next_set = set()
            for s in maybe_unreachable_set:
                # Check if the state is reachable, if not remove the state and
                # fill the next_set with all outgoing edges.
                if len(self.states[s].backedges) == 0 and s not in init:
                    self.remove_state(s, next_set)
            maybe_unreachable_set = next_set

    def is_reachable_state(self, s):
        """Check whether the current state is reachable or not."""
        assert isinstance(s, int)
        if self.states[s] is None:
            return False
        reachable_back = set()
        todo = [s]
        while todo:
            s = todo.pop()
            reachable_back.add(s)
            for edge in self.states[s].backedges:
                if edge.src not in reachable_back:
                    todo.append(edge.src)
        for _, s in self.named_goals:
            if s in reachable_back:
                return True
        return False

    def debug_dump(self):
        # Sort the grammar by state hash, such that it can be compared
        # before/after grammar modifications.
        temp = [s for s in self.states if s is not None]
        temp = sorted(temp, key=lambda s: s.stable_hash)
        for s in temp:
            print(s.stable_str(self.states))

    def get_flag_for(self, nts):
        nts = OrderedFrozenSet(nts)
        self.flags.add(nts)
        return "_".join(["flag", *nts])

    def create_lr0_table(self, grammar, verbose, progress):
        if verbose or progress:
            print("Create LR(0) parse table.")
        assert isinstance(grammar, CanonicalGrammar)
        assert isinstance(grammar.grammar, Grammar)

        goals = grammar.grammar.goals()
        self.named_goals = []

        # Temporarily record tuples of (LR0Generator, StateAndTransition)
        # objects used for visiting the grammar.
        todo = collections.deque()
        # Record the starting goals in the todo list.
        for nt in goals:
            init_nt = Nt(InitNt(nt), ())
            it = LR0Generator.start(grammar, init_nt)
            s = self.get_state(it.stable_locations())
            todo.append((it, s))
            self.named_goals.append((nt, s.index))

        # Iterate the grammar with sets of LR Items abstracted by the
        # LR0Generator, and create new states in the parse table as long as new
        # sets of LR Items are discovered.
        def visit_grammar():
            while todo:
                yield  # progress bar.
                # TODO: Compare stack / queue, for the traversal of the states.
                s_it, s = todo.popleft()
                if verbose:
                    print("\nMapping state {} to LR0:\n{}".format(s.stable_hash, s_it))
                for k, sk_it in s_it.transitions().items():
                    locations = sk_it.stable_locations()
                    if not self.is_term_shifted(k):
                        locations = OrderedFrozenSet()
                    is_new, sk = self.new_state(locations)
                    if is_new:
                        todo.append((sk_it, sk))

                    # Add the edge from s to sk with k.
                    self.add_edge(s, k, sk.index)
        consume(visit_grammar(), progress)

        if verbose:
            print("Create LR(0) Table Result:")
            self.debug_dump()

    def is_term_shifted(self, term):
        return not (isinstance(term, Action) and term.update_stack())

    def is_valid_path(self, path, state=None):
        """This function is used to check a list of edges and returns whether it
        corresponds to a valid path within the parse table. This is useful when
        merging sequences of edges from various locations."""
        if not state and path != []:
            state = path[0].src
        while path:
            edge = path[0]
            path = path[1:]
            if state != edge.src:
                return False
            term = edge.term
            if term is None and len(path) == 0:
                return True

            row = self.states[state]
            if term not in row:
                return False
            state = row[term]
        return True

    def shifted_path_to(self, n, right_of):
        "Compute all paths with n shifted terms, ending with right_of."
        assert isinstance(right_of, list) and len(right_of) >= 1
        if n == 0:
            yield right_of
        state = right_of[0].src
        assert isinstance(state, int)
        for edge in self.states[state].backedges:
            if not self.is_term_shifted(edge.term):
                print(repr(edge))
                print(self.states[edge.src])
            assert self.is_term_shifted(edge.term)
            if self.term_is_stacked(edge.term):
                s_n = n - 1
                if n == 0:
                    continue
            else:
                s_n = n
            from_edge = Edge(edge.src, edge.term)
            for path in self.shifted_path_to(s_n, [from_edge] + right_of):
                yield path

    def reduce_path(self, shifted):
        """Compute all paths which might be reduced by a given action. This function
        assumes that the state is reachable from the starting goals, and that
        the depth which is being queried has valid answers."""
        assert len(shifted) >= 1
        action = shifted[-1].term
        assert action.update_stack()
        reducer = action.reduce_with()
        assert isinstance(reducer, Reduce)
        nt = reducer.nt
        depth = reducer.pop + reducer.replay
        if depth > 0:
            # We are reducing at least one element from the stack.
            stacked = [i for i, e in enumerate(shifted) if self.term_is_stacked(e.term)]
            if len(stacked) < depth:
                # We have not shifted enough elements to cover the full reduce
                # rule, start looking for context using backedges.
                shifted_from = 0
                depth -= len(stacked)
            else:
                # We shifted much more elements than necessary for reducing,
                # just start from the first stacked element which correspond to
                # consuming all stack element reduced.
                shifted_from = stacked[-depth]
                depth = 0
            shifted_end = shifted[shifted_from:]
        else:
            # We are reducing no element from the stack.
            shifted_end = shifted[-1:]
        for path in self.shifted_path_to(depth, shifted_end):
            # NOTE: When reducing, we might be tempted to verify that the
            # reduced non-terminal is part of the state we are reducing to, and
            # it surely is for one of the shifted path. However, this would be
            # an error in an inconsistent grammar. (see issue #464)
            #
            # Thus, we might yield plenty of path which are not reducing the
            # expected non-terminal, but these are expected to be filtered out
            # by the APS, as the inability of shifting these non-terminals
            # would remove these cases.
            assert self.assume_inconsistent or nt in self.states[path[0].src].nonterminals
            yield path

    def term_is_stacked(self, term):
        return not isinstance(term, Action)

    def aps_visitor(self, aps, visit):
        """Visit all the states of the parse table, as-if we were running a
        Generalized LR parser.

        However, instead parsing content, we use this algorithm to generate
        both the content which remains to be parsed as well as the context
        which might lead us to be in the state which from which we started.

        This algorithm takes an APS (Abstract Parser State) and a callback, and
        consider all edges of the parse table, unless restricted by one of the
        previously encountered actions. These restrictions, such as replayed
        lookahead or the path which might be reduced are used for filtering out
        states which are not handled by this parse table.

        For each edge, this functions calls the visit functions to know whether
        to stop or continue. The visit function might capture APS given as
        argument to be used for other analysis.

        """
        todo = []
        todo.append(aps)
        while todo:
            aps = todo.pop()
            cont = visit(aps)
            if not cont:
                continue
            todo.extend(aps.shift_next(self))

    def lanes(self, state):
        """Compute the lanes from the amount of lookahead available. Only consider
        context when reduce states are encountered."""
        assert isinstance(state, int)
        record = []

        def visit(aps):
            has_shift_loop = len(aps.shift) != 1 + len(set(zip(aps.shift, aps.shift[1:])))
            has_stack_loop = len(aps.stack) != 1 + len(set(zip(aps.stack, aps.stack[1:])))
            has_lookahead = len(aps.lookahead) >= 1
            stop = has_shift_loop or has_stack_loop or has_lookahead
            # print("\t{} = {} or {} or {}".format(
            #     stop, has_shift_loop, has_stack_loop, has_lookahead))
            # print(aps_str(aps, "\tvisitor"))
            if stop:
                print("lanes_visit stop:")
                print(aps_str(aps, "\trecord"))
                record.append(aps)
            return not stop
        self.aps_visitor(APS.start(state), visit)
        return record

    def context_lanes(self, state):
        """Compute lanes, such that each reduce action can have set of unique stack to
        reach the given state. The stacks are assumed to be loop-free by
        reducing edges at most once.

        In order to avoid attempting to eagerly solve everything using context
        information, we break this loop as soon as we have one token of
        lookahead in a case which does not have enough context.

        The return value is a tuple where the first element is a boolean which
        is True if we should fallback on solving this issue with more
        lookahead, and the second is the list of APS lanes which are providing
        enough context to disambiguate the inconsistency of the given state."""

        assert isinstance(state, int)

        def not_interesting(aps):
            reduce_list = [e for e in aps.history if self.is_term_shifted(e.term)]
            has_reduce_loop = len(reduce_list) != len(set(reduce_list))
            return has_reduce_loop

        # The context is a dictionary which maps all stack suffixes from an APS
        # stack. It is mapped to a list of tuples, where the each tuple is the
        # index with the APS stack and the APS action used to follow this path.
        context = collections.defaultdict(lambda: [])

        def has_enough_context(aps):
            try:
                assert aps.history[0] in context[tuple(aps.stack)]
                # Check the number of different actions which can reach this
                # location. If there is more than 1, then we do not have enough
                # context.
                return len(set(context[tuple(aps.stack)])) <= 1
            except IndexError:
                return False

        collect = [APS.start(state)]
        enough_context = False
        while not enough_context:
            # print("collect.len = {}".format(len(collect)))
            # Fill the context dictionary with all the sub-stack which might be
            # encountered by other APS.
            recurse = []
            context = collections.defaultdict(lambda: [])
            while collect:
                aps = collect.pop()
                recurse.append(aps)
                if aps.history == []:
                    continue
                for i in range(len(aps.stack)):
                    context[tuple(aps.stack[i:])].append(aps.history[0])
            assert collect == []

            # print("recurse.len = {}".format(len(recurse)))
            # Iterate over APS which do not yet have enough context information
            # to uniquely identify a single action.
            enough_context = True
            while recurse:
                aps = recurse.pop()
                if not_interesting(aps):
                    # print("discard uninteresting context lane:")
                    # print(aps_str(aps, "\tcontext"))
                    continue
                if has_enough_context(aps):
                    collect.append(aps)
                    continue
                # If we have not enough context but some lookahead is
                # available, attempt to first solve this issue using more
                # lookahead before attempting to use context information.
                if len(aps.lookahead) >= 1:
                    # print("discard context_lanes due to lookahead:")
                    # for aps in itertools.chain(collect, recurse, [aps]):
                    #     print(aps_str(aps, "\tcontext"))
                    return True, []
                enough_context = False
                # print("extend starting at:\n{}".format(aps_str(aps, "\tcontext")))
                collect.extend(aps.shift_next(self))
            assert recurse == []

        # print("context_lanes:")
        # for aps in collect:
        #     print(aps_str(aps, "\tcontext"))

        return False, collect

    def lookahead_lanes(self, state):
        """Compute lanes to collect all lookahead symbols available. After each reduce
        action, there is no need to consider the same non-terminal multiple
        times, we are only interested in lookahead token and not in the context
        information provided by reducing action."""
        assert isinstance(state, int)
        record = []
        # After the first reduce action, we do not want to spend too much
        # resource visiting edges which would give us the same information.
        # Therefore, if we already reduce an action to a given state, then we
        # skip looking for lookahead that we already visited.
        #
        # Set of (first-reduce-edge, reducing-base, last-reduce-edge)
        seen_edge_after_reduce = set()

        def find_first_reduce(edges):
            for i, edge in enumerate(edges):
                if not self.is_term_shifted(edge.term):
                    return i, edge
            return 0, None

        def find_last_reduce(edges):
            for i, edge in zip(reversed(range(len(edges))), reversed(edges)):
                if not self.is_term_shifted(edge.term):
                    return i, edge
            return 0, None

        def visit(aps):
            # Note, this suppose that we are not considering flags when
            # computing, as flag might prevent some lookahead investigations.
            reduce_key = None
            first_index, first_reduce = find_first_reduce(aps.history)
            last_index, last_reduce = find_last_reduce(aps.history)
            if first_index != last_index and first_reduce and last_reduce:
                if not isinstance(aps.history[-1].term, Action):
                    reduce_key = (first_reduce, aps.shift[0].src, last_reduce.term)
            has_seen_edge_after_reduce = reduce_key and reduce_key in seen_edge_after_reduce
            has_lookahead = len(aps.lookahead) >= 1
            stop = has_seen_edge_after_reduce or has_lookahead
            # print("stop: {}, size lookahead: {}, seen_edge_after_reduce: {}".format(
            #     stop, len(aps.lookahead), repr(reduce_key)
            # ))
            # print(aps.string("\tvisitor"))
            if stop:
                if has_lookahead:
                    record.append(aps)
            if reduce_key:
                seen_edge_after_reduce.add(reduce_key)
            return not stop
        self.aps_visitor(APS.start(state), visit)
        return record

    def fix_with_context(self, s, aps_lanes):
        raise ValueError("fix_with_context: Not Implemented")
    #     # This strategy is about using context information. By using chains of
    #     # reduce actions, we are able to increase the knowledge of the stack
    #     # content. The stack content is the context which can be used to
    #     # determine how to consider a reduction. The stack content is also
    #     # called a lane, as defined in the Lane Table algorithm.
    #     #
    #     # To add context information to the current graph, we add flags
    #     # manipulation actions.
    #     #
    #     # Consider each edge as having an implicit function which can map one
    #     # flag value to another. The following implements a unification
    #     # algorithm which is attempting to solve the question of what is the
    #     # flag value, and where it should be changed.
    #     #
    #     # NOTE: (nbp) I would not be surprised if there is a more specialized
    #     # algorithm, but I failed to find one so far, and this problem
    #     # definitely looks like a unification problem.
    #     Id = collections.namedtuple("Id", "edge")
    #     Eq = collections.namedtuple("Eq", "flag_in edge flag_out")
    #     Var = collections.namedtuple("Var", "n")
    #     SubSt = collections.namedtuple("SubSt", "var by")
    #
    #     # Unify expression, and return one substitution if both expressions
    #     # can be unified.
    #     def unify_expr(expr1, expr2, swapped=False):
    #         if isinstance(expr1, Eq) and isinstance(expr2, Id):
    #             if expr1.edge != expr2.edge:
    #                 # Different edges are ok, but produce no substituions.
    #                 return True
    #             if isinstance(expr1.flag_in, Var):
    #                 return SubSt(expr1.flag_in, expr1.flag_out)
    #             if isinstance(expr1.flag_out, Var):
    #                 return SubSt(expr1.flag_out, expr1.flag_in)
    #             # We are unifying with a relation which consider the current
    #             # function as an identity function. Having different values as
    #             # input and output fails the unification rule.
    #             return expr1.flag_out == expr1.flag_in
    #         if isinstance(expr1, Eq) and isinstance(expr2, Eq):
    #             if expr1.edge != expr2.edge:
    #                 # Different edges are ok, but produce no substituions.
    #                 return True
    #             if expr1.flag_in is None and isinstance(expr2.flag_in, Var):
    #                 return SubSt(expr2.flag_in, None)
    #             if expr1.flag_out is None and isinstance(expr2.flag_out, Var):
    #                 return SubSt(expr2.flag_out, None)
    #             if expr1.flag_in == expr2.flag_in:
    #                 if isinstance(expr1.flag_out, Var):
    #                     return SubSt(expr1.flag_out, expr2.flag_out)
    #                 elif isinstance(expr2.flag_out, Var):
    #                     return SubSt(expr2.flag_out, expr1.flag_out)
    #                 # Reject solutions which are not deterministic. We do not
    #                 # want the same input flag to have multiple outputs.
    #                 return expr1.flag_out == expr2.flag_out
    #             if expr1.flag_out == expr2.flag_out:
    #                 if isinstance(expr1.flag_in, Var):
    #                     return SubSt(expr1.flag_in, expr2.flag_in)
    #                 elif isinstance(expr2.flag_in, Var):
    #                     return SubSt(expr2.flag_in, expr1.flag_in)
    #                 return True
    #         if not swapped:
    #             return unify_expr(expr2, expr1, True)
    #         return True
    #
    #     # Apply substituion rule to an expression.
    #     def subst_expr(subst, expr):
    #         if expr == subst.var:
    #             return True, subst.by
    #         if isinstance(expr, Eq):
    #             subst1, flag_in = subst_expr(subst, expr.flag_in)
    #             subst2, flag_out = subst_expr(subst, expr.flag_out)
    #             return subst1 or subst2, Eq(flag_in, expr.edge, flag_out)
    #         return False, expr
    #
    #     # Add an expression to an existing knowledge based which is relying on
    #     # a set of free variables.
    #     def unify_with(expr, knowledge, free_vars):
    #         old_knowledge = knowledge
    #         old_free_Vars = free_vars
    #         while True:
    #             subst = None
    #             for rel in knowledge:
    #                 subst = unify_expr(rel, expr)
    #                 if subst is False:
    #                     raise Error("Failed to find a coherent solution")
    #                 if subst is True:
    #                     continue
    #                 break
    #             else:
    #                 return knowledge + [expr], free_vars
    #             free_vars = [fv for fv in free_vars if fv != subst.var]
    #             # Substitue variables, and re-add rules which have substituted
    #             # vars to check the changes to other rules, in case 2 rules are
    #             # now in conflict or in case we can propagate more variable
    #             # changes.
    #             subst_rules = [subst_expr(subst, k) for k in knowledge]
    #             knowledge = [rule for changed, rule in subst_rule if not changed]
    #             for changed, rule in subst_rule:
    #                 if not changed:
    #                     continue
    #                 knowledge, free_vars = unify_with(rule, knowledge, free_vars)
    #
    #     # Register boundary conditions as part of the knowledge based, i-e that
    #     # reduce actions are expecting to see the flag value matching the
    #     # reduced non-terminal, and that we have no flag value at the start of
    #     # every lane head.
    #     #
    #     # TODO: Catch exceptions from the unify function in case we do not yet
    #     # have enough context to disambiguate.
    #     rules = []
    #     free_vars = []
    #     last_free = 0
    #     maybe_id_edges = set()
    #     nts = set()
    #     for aps in aps_lanes:
    #         assert len(aps.stack) >= 1
    #         flag_in = None
    #         for edge in aps.stack[-1]:
    #             i = last_free
    #             last_free += 1
    #             free_vars.append(Var(i))
    #             rule = Eq(flag_in, edge, Var(i))
    #             rules, free_vars = unify_with(rule, rules, free_vars)
    #             flag_in = Var(i)
    #             if flag_in is not None:
    #                 maybe_id_edges.add(Id(edge))
    #         edge = aps.stack[-1]
    #         nt = edge.term.reduce_with().nt
    #         rule = Eq(nt, edge, None)
    #         rules, free_vars = unify_with(rule, rules, free_vars)
    #         nts.add(nt)
    #
    #     # We want to produce a parse table where most of the node are ignoring
    #     # the content of the flag which is being added. Thus we want to find a
    #     # solution where most edges are the identical function.
    #     def fill_with_id_functions(rules, free_vars, maybe_id_edges):
    #         min_rules, min_vars = rules, free_vars
    #         for num_id_edges in reversed(range(len(maybe_id_edges))):
    #             for id_edges in itertools.combinations(edges, num_id_edges):
    #                 for edge in id_edges:
    #                     new_rules, new_free_vars = unify_with(rule, rules, free_vars)
    #                     if new_free_vars == []:
    #                         return new_rules, new_free_vars
    #                     if len(new_free_vars) < len(min_free_vars):
    #                         min_vars = new_free_vars
    #                         min_rules = new_rules
    #         return rules, free_vars
    #
    #     rules, free_vars = fill_with_id_functions(rules, free_vars, maybe_id_edges)
    #     if free_vars != []:
    #         raise Error("Hum … maybe we can try to iterate over the remaining free-variable.")
    #     print("debug: Great we found a solution for a reduce-reduce conflict")
    #
    #     # The set of rules describe the function that each edge is expected to
    #     # support. If there is an Id(edge), then we know that we do not have to
    #     # change the graph for the given edge. If the rule is Eq(A, edge, B),
    #     # then we have to (filter A & pop) and push B, except if A or B is
    #     # None.
    #     #
    #     # For each edge, collect the set of rules concerning the edge to
    #     # determine which edges have to be transformed to add the filter&pop
    #     # and push actions.
    #     edge_rules = collections.defaultdict(lambda: [])
    #     for rule in rules:
    #         if isinstance(rule, Id):
    #             edge_rules[rule.edge] = None
    #         elif isinstance(rule, Eq):
    #             if edge_rules[rule.edge] is not None:
    #                 edge_rules[rule.edge].append(rule)
    #
    #     maybe_unreachable_set = set()
    #     flag_name = self.get_flag_for(nts)
    #     for edge, rules in edge_rules.items():
    #         # If the edge is an identity function, then skip doing any
    #         # modifications on it.
    #         if rules is None:
    #             continue
    #         # Otherwise, create a new state and transition for each mapping.
    #         src = self.states[edge.src]
    #         dest = src[edge.term]
    #         dest_state = self.states[dest]
    #         # TODO: Add some information to avoid having identical hashes as
    #         # the destination.
    #         actions = []
    #         for rule in OrderedFrozenSet(rules):
    #             assert isinstance(rule, Eq)
    #             seq = []
    #             if rule.flag_in is not None:
    #                 seq.append(FilterFlag(flag_name, True))
    #                 if rule.flag_in != rule.flag_out:
    #                     seq.append(PopFlag(flag_name))
    #             if rule.flag_out is not None and rule.flag_in != rule.flag_out:
    #                 seq.append(PushFlag(flag_name, rule.flag_out))
    #             actions.append(Seq(seq))
    #         # Assert that we do not map flag_in more than once.
    #         assert len(set(eq.flag_in for eq in rules)) < len(rules)
    #         # Create the new state and add edges.
    #         is_new, switch = self.new_state(dest.locations, OrderedFrozenSet(actions))
    #         assert is_new
    #         for seq in actions:
    #             self.add_edge(switch, seq, dest)
    #
    #         # Replace the edge from src to dest, by an edge from src to the
    #         # newly created switch state, which then decide which flag to set
    #         # before going to the destination target.
    #         self.replace_edge(src, edge.term, switch, maybe_unreachable_set)
    #
    #     self.remove_unreachable_states(maybe_unreachable_set)
    #     pass

    def fix_with_lookahead(self, s, aps_lanes):
        # Find the list of terminals following each actions (even reduce
        # actions).
        assert all(len(aps.lookahead) >= 1 for aps in aps_lanes)
        if self.debug_info:
            for aps in aps_lanes:
                print(str(aps))
        maybe_unreachable_set = set()
        # For each shifted term, associate a set of state and actions which
        # would have to be executed.
        shift_map = collections.defaultdict(lambda: [])
        for aps in aps_lanes:
            actions = aps.history
            assert isinstance(actions[-1], Edge)
            assert actions[-1].term == aps.lookahead[0]
            src = actions[-1].src
            term = actions[-1].term
            # No need to consider any action behind the first reduced action
            # since the reduced action is in charge of replaying the lookahead
            # terms.
            actions = list(keep_until(actions[:-1], lambda edge: not self.is_term_shifted(edge.term)))
            assert all(isinstance(edge.term, Action) for edge in actions)

            # Change the order of the shifted term, shift all actions by 1 with
            # the given lookahead term, in order to match the newly generated
            # state machine.
            #
            # Shifting actions with the list of shifted terms is used to record
            # the number of terms to be replayed, as well as verifying whether
            # Lookahead filter actions should accept or reject this lane.
            new_actions = []
            accept = True
            for edge in actions:
                new_term = edge.term.shifted_action(term)
                if new_term is False:
                    accept = False
                    break
                elif new_term is True:
                    continue
                new_actions.append(Edge(edge.src, new_term))
            if accept:
                target = self.states[src][term]
                target = self.states[target]
                shift_map[term].append((target, new_actions))

        # Restore the new state machine based on a given state to use as a base
        # and the shift_map corresponding to edges.
        def restore_edges(state, shift_map, depth):
            assert isinstance(state, StateAndTransitions)
            # print("{}starting with {}\n".format(depth, state))
            edges = {}
            for term, actions_list in shift_map.items():
                # print("{}term: {}, lists: {}\n".format(depth, repr(term), repr(actions_list)))
                # Collect all the states reachable after shifting the term.
                # Compute the unique name, based on the locations and actions
                # which are delayed.
                locations = OrderedSet()
                delayed = OrderedSet()
                new_shift_map = collections.defaultdict(lambda: [])
                recurse = False
                if not self.is_term_shifted(term):
                    # There is no more target after a reduce action.
                    actions_list = []
                for target, actions in actions_list:
                    assert isinstance(target, StateAndTransitions)
                    locations |= target.locations
                    delayed |= target.delayed_actions
                    if actions != []:
                        # Pull edges, with delayed actions.
                        edge = actions[0]
                        assert isinstance(edge, Edge)
                        for action in actions:
                            delayed.add(action.term)
                        new_shift_map[edge.term].append((target, actions[1:]))
                        recurse = True
                    else:
                        # Pull edges, as a copy of existing edges.
                        for next_term, next_dest in target.edges():
                            next_dest = self.states[next_dest]
                            new_shift_map[next_term].append((next_dest, []))

                locations = OrderedFrozenSet(locations)
                delayed = OrderedFrozenSet(delayed)
                is_new, new_target = self.new_state(locations, delayed)
                edges[term] = new_target.index
                if self.debug_info:
                    print("{}is_new = {}, index = {}".format(depth, is_new, new_target.index))
                    print("{}Add: {} -- {} --> {}".format(depth, state.index, str(term), new_target.index))
                    print("{}continue: (is_new: {}) or (recurse: {})".format(depth, is_new, recurse))
                if is_new or recurse:
                    restore_edges(new_target, new_shift_map, depth + "  ")

            self.clear_edges(state, maybe_unreachable_set)
            for term, target in edges.items():
                self.add_edge(state, term, target)
            if self.debug_info:
                print("{}replaced by {}\n".format(depth, state))

        state = self.states[s]
        restore_edges(state, shift_map, "")
        self.remove_unreachable_states(maybe_unreachable_set)

    def fix_inconsistent_state(self, s, verbose):
        # Fix inconsistent states works one state at a time. The goal is to
        # achieve the same method as the Lane Tracer, but instead of building a
        # table to then mutate the parse state, we mutate the parse state
        # directly.
        #
        # This strategy is simpler, and should be able to reproduce the same
        # graph mutations as seen with Lane Table algorithm. One of the problem
        # with the Lane Table algorithm is that it assume reduce operations,
        # and as such it does not apply simply to epsilon transitions which are
        # used as conditions on the parse table.
        #
        # By using push-flag and filter-flag actions, we are capable to
        # decompose the Lane Table transformation of the parse table into
        # multiple steps which are working one step at a time, and with less
        # table state duplication.

        state = self.states[s]
        if state is None or not state.is_inconsistent():
            return False

        all_reduce = all(a.update_stack() for a, _ in state.epsilon)
        any_shift = (len(state.terminals) + len(state.nonterminals) + len(state.errors)) > 0
        try_with_context = all_reduce and not any_shift
        try_with_lookahead = not try_with_context
        # if verbose:
        #     print(aps_lanes_str(aps_lanes, "fix_inconsistent_state:", "\taps"))
        if try_with_context:
            if verbose:
                print("\tFix with context.")
            try_with_lookahead, aps_lanes = self.context_lanes(s)
            if not try_with_lookahead:
                assert aps_lanes != []
                self.fix_with_context(s, aps_lanes)
            elif verbose:
                print("\tFallback on fixing with lookahead.")
        if try_with_lookahead:
            if verbose:
                print("\tFix with lookahead.")
            aps_lanes = self.lookahead_lanes(s)
            assert aps_lanes != []
            self.fix_with_lookahead(s, aps_lanes)
        return True

    def fix_inconsistent_table(self, verbose, progress):
        """The parse table might be inconsistent. We fix the parse table by looking
        around the inconsistent states for more context. Either by looking at the
        potential stack state which might lead to the inconsistent state, or by
        increasing the lookahead."""
        self.assume_inconsistent = True
        if verbose or progress:
            print("Fix parse table inconsistencies.")

        todo = collections.deque()
        for state in self.states:
            if state.is_inconsistent():
                todo.append(state.index)

        if verbose and todo:
            print("\n".join([
                "\nGrammar is inconsistent.",
                "\tNumber of States = {}",
                "\tNumber of inconsistencies found = {}"]).format(
                    len(self.states), len(todo)))

        count = 0

        def visit_table():
            nonlocal count
            unreachable = []
            while todo:
                while todo:
                    yield  # progress bar.
                    # TODO: Compare stack / queue, for the traversal of the states.
                    s = todo.popleft()
                    if not self.is_reachable_state(s):
                        # NOTE: We do not fix unreachable states, as we might
                        # not be able to compute the reduce actions. However,
                        # we should not clean edges not backedges as the state
                        # might become reachable later on, since states are
                        # shared if they have the same locations.
                        unreachable.append(s)
                        continue
                    assert self.states[s].is_inconsistent()
                    start_len = len(self.states)
                    if verbose:
                        count = count + 1
                        print("Fixing state {}\n".format(self.states[s].stable_str(self.states)))
                    try:
                        self.fix_inconsistent_state(s, verbose)
                    except Exception as exc:
                        self.debug_info = True
                        raise ValueError(
                            "Error while fixing conflict in state {}\n\n"
                            "In the following grammar productions:\n{}"
                            .format(self.states[s].stable_str(self.states), self.debug_context(s, "\n", "\t"))
                        ) from exc
                    new_inconsistent_states = [
                        s.index for s in self.states[start_len:]
                        if s.is_inconsistent()
                    ]
                    if verbose:
                        print("\tAdding {} states".format(len(self.states[start_len:])))
                        print("\tWith {} inconsistent states".format(len(new_inconsistent_states)))
                    todo.extend(new_inconsistent_states)

                # Check whether none of the previously inconsistent and
                # unreahable state became reachable. If so add it back to the
                # todo list.
                still_unreachable = []
                for s in unreachable:
                    if self.is_reachable_state(s):
                        todo.append(s)
                    else:
                        still_unreachable.append(s)
                unreachable = still_unreachable

        consume(visit_table(), progress)
        if verbose:
            print("\n".join([
                "\nGrammar is now consistent.",
                "\tNumber of States = {}",
                "\tNumber of inconsistencies solved = {}"]).format(
                    len(self.states), count))
        assert not self.is_inconsistent()
        self.assume_inconsistent = False

        if verbose:
            print("Fix Inconsistent Table Result:")
            self.debug_dump()

    def remove_all_unreachable_state(self, verbose, progress):
        self.states = [s for s in self.states if s is not None]
        state_map = {s.index: i for i, s in enumerate(self.states)}
        for s in self.states:
            s.rewrite_state_indexes(state_map)

    def fold_identical_endings(self, verbose, progress):
        # If 2 states have the same outgoing edges, then we can merge the 2
        # states into a single state, and rewrite all the backedges leading to
        # these states to be replaced by edges going to the reference state.
        if verbose or progress:
            print("Fold identical endings.")
        maybe_unreachable = set()

        def rewrite_backedges(state_list):
            # All states have the same outgoing edges. Thus we replace all of
            # them by a single state.
            ref = state_list[0]
            replace_edges = [e for s in state_list[1:] for e in s.backedges]
            hit = False
            for edge in replace_edges:
                src = self.states[edge.src]
                # print("replace {} -- {} --> {}, by {} -- {} --> {}"
                #       .format(src.index, term, src[term], src.index, term, ref.index))
                self.replace_edge(src, edge.term, ref.index, maybe_unreachable)
                hit = True
            return hit

        def rewrite_if_same_outedges(state_list):
            outedges = collections.defaultdict(lambda: [])
            for s in state_list:
                outedges[tuple(s.edges())].append(s)
            hit = False
            for same in outedges.values():
                if len(same) > 1:
                    hit = rewrite_backedges(same) or hit
            return hit

        def visit_table():
            hit = True
            while hit:
                yield  # progress bar.
                hit = rewrite_if_same_outedges(self.states)
        consume(visit_table(), progress)

        self.remove_unreachable_states(maybe_unreachable)
        self.remove_all_unreachable_state(verbose, progress)

    def group_epsilon_states(self, verbose, progress):
        shift_states = [s for s in self.states if len(s.epsilon) == 0]
        action_states = [s for s in self.states if len(s.epsilon) > 0]
        self.states = []
        self.states.extend(shift_states)
        self.states.extend(action_states)
        state_map = {s.index: i for i, s in enumerate(self.states)}
        for s in self.states:
            s.rewrite_state_indexes(state_map)

    def count_shift_states(self):
        return sum(1 for s in self.states if len(s.epsilon) == 0)

    def count_action_states(self):
        return sum(1 for s in self.states if len(s.epsilon) > 0)

    def prepare_debug_context(self):
        """To better filter out the traversal of the grammar in debug context, we
        pre-compute for each state the maximal depth of each state within a
        production. Therefore, if visiting a state no increases the reducing
        depth beyind the ability to shrink the shift list to 0, then we can
        stop going deeper, as we entered a different production. """
        depths = collections.defaultdict(lambda: [])
        for s in self.states:
            if s is None or not s.epsilon:
                continue
            aps = APS.start(s.index)
            for aps_next in aps.shift_next(self):
                if not aps_next.reducing:
                    continue
                for i, edge in enumerate(aps_next.stack):
                    depths[edge.src].append(i + 1)
        depths = { s: max(ds) for s, ds in depths.items() }
        return depths

    def debug_context(self, state, split_txt="; ", prefix=""):
        "Reconstruct the grammar production by traversing the parse table."
        assert isinstance(state, int)
        if self.debug_info is False:
            return ""
        if self.debug_info is True:
            self.debug_info = self.prepare_debug_context()
        record = []

        def visit(aps):
            # Stop after reducing once.
            if aps.history == []:
                return True
            last = aps.history[-1].term
            is_reduce = not self.is_term_shifted(last)
            has_shift_loop = len(aps.shift) != 1 + len(set(zip(aps.shift, aps.shift[1:])))
            can_reduce_later = True
            try:
                can_reduce_later = self.debug_info[aps.shift[-1].src] >= len(aps.shift)
            except KeyError:
                can_reduce_later = False
            stop = is_reduce or has_shift_loop or not can_reduce_later
            # Record state which are reducing at most all the shifted states.
            save = stop and len(aps.shift) == 1
            save = save and is_reduce
            save = save and last.reduce_with().nt in self.states[aps.shift[0].src]
            if save:
                record.append(aps)
            return not stop
        self.aps_visitor(APS.start(state), visit)

        context = OrderedSet()
        for aps in record:
            assert aps.history != []
            assert aps.history[-1].term.update_stack()
            reducer = aps.history[-1].term.reduce_with()
            replay = reducer.replay
            before = [repr(e.term) for e in aps.stack[:-1]]
            after = [repr(e.term) for e in aps.history[:-1]]
            prod = before + ["\N{MIDDLE DOT}"] + after
            if replay < len(after) and replay > 0:
                del prod[-replay:]
                replay = 0
            if replay > len(after):
                replay += 1
            if replay > 0:
                prod = prod[:-replay] + ["[lookahead:"] + prod[-replay:] + ["]"]
            txt = "{}{} ::= {}".format(
                prefix,
                repr(aps.history[-1].term.reduce_with().nt),
                " ".join(prod)
            )
            context.add(txt)

        if split_txt is None:
            return context
        return split_txt.join(txt for txt in sorted(context))


def generate_parser_states(
        grammar: Grammar,
        *,
        verbose: bool = False,
        progress: bool = False,
        debug: bool = False
) -> ParseTable:
    parse_table = ParseTable(CanonicalGrammar(grammar), verbose, progress, debug)
    return parse_table


def generate_parser(
        out: io.TextIOBase,
        source: Grammar,
        *,
        verbose: bool = False,
        progress: bool = False,
        debug: bool = False,
        target: str = 'python',
        handler_info: typing.Any = None
) -> None:
    assert target in ('python', 'rust')

    if isinstance(source, Grammar):
        grammar = CanonicalGrammar(source)
        parser_data = generate_parser_states(
            source, verbose=verbose, progress=progress, debug=debug)
    elif isinstance(source, ParseTable):
        parser_data = source
        parser_data.debug_info = debug
    else:
        raise TypeError("unrecognized source: {!r}".format(source))

    if target == 'rust':
        if isinstance(parser_data, ParseTable):
            emit.write_rust_parse_table(out, parser_data, handler_info)
        else:
            raise ValueError("Unexpected parser_data kind")
    else:
        if isinstance(parser_data, ParseTable):
            emit.write_python_parse_table(out, parser_data)
        else:
            raise ValueError("Unexpected parser_data kind")


def compile(grammar, verbose=False, debug=False):
    assert isinstance(grammar, Grammar)
    out = io.StringIO()
    generate_parser(out, grammar, verbose=verbose, debug=debug)
    scope = {}
    if verbose:
        with open("parse_with_python.py", "w") as f:
            f.write(out.getvalue())
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
    Parser = sandbox['Parser']

    while True:
        try:
            line = input('> ')
        except EOFError:
            break

        try:
            parser = Parser()
            lexer = tokenize(parser)
            lexer.write(line)
            result = lexer.close()
        except Exception as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
        else:
            print(result)


if __name__ == '__main__':
    demo()
