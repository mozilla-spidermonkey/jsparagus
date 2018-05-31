#!/usr/bin/env python3

"""gen.py - Third stab at a parser generator.

**Nature of a grammar.**
A grammar is a dictionary {str: [[symbol]]} mapping names of nonterminals to lists of productions.
A production is a nonempty list of symbols.
Each symbol specifies either a kind of terminal, a nonterminal (by name),
or a Reduction (a namedtuple that guides the construction of the parse tree).

**Context of the generated parser.**
The user passes to each method an object representing the input sequence.
This object must support two methods:

*   `src.peek()` returns the kind of the next token, or `None` at the end of input.

*   `src.take(kind)` throws an exception if `src.peek() != kind`;
    otherwise, it removes the next token from the input stream and returns it.
    The special case `src.take(None)` checks that the input stream is empty:
    if so, it returns None; if not, it throws.

**Simplifying assumptions about the grammar.**
We assume the grammar is left-factored, or will be once we eliminate left recursion.
We verify that the grammar, after eliminating left recursion, is LL(1).

We assume that no production in the input grammar matches the empty
string. (However, eliminating left-recursion typically creates productions that
match the empty string, so it's unclear what this buys us, except that the
dragon book claims the algorithm to eliminate left-recursion only works if we
have no such productions to begin with—a claim I don't understand.)

We assume that every nonterminal matches at least one string of finite length.
It's not a bug if it doesn't, but it would be nice to check.
"""

import collections
import pprint, textwrap

from pgen_runtime import is_terminal, is_nt, is_reduction, Reduction

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

def eliminate_left_recursion(grammar):
    """Dragon book Algorithm 4.1."""

    def quasisort_nts():
        """ Make a half-hearted effort to put all the nts in a nice order for processing.

        Since the algorithm bans left-calls from later nts to earlier ones,
        the list should ideally be arranged so that such calls are rare; that is,
        as much as possible, if A left-calls B, then A should appear before B.

        We do a sort of topological sort to try to ensure this; but cycles are possible,
        and when we find one, we leave a left call that will be eliminated by a future
        call to eliminate_left_calls().
        """
        out = []
        stack = []
        def visit(nt):
            if nt not in out:
                stack.append(nt)
                for r in grammar[nt]:
                    if r and is_nt(r[0]):
                        if r[0] in stack:
                            pass # oh well
                        else:
                            visit(r[0])
                out.append(nt)
                stack.pop()

        for nt in grammar:
            visit(nt)

        assert sorted(grammar) == sorted(out)
        out.reverse()
        return out

    def eliminate_left_calls(from_nt, to_nt):
        """Rewrite productions of `from_nt` so that none start with the symbol `to_nt`.
        This is done by inlining all productions of `to_nt` into `from_nt`. (The result
        could be a combinatorial explosion, but in practice it's not that bad.)

        from_nt ::= to_nt a0 | ... | b0 | ...
        ==> from_nt ::= c0 a0 | ... | b0 | ...     where to_nt ::= c0 | ...

        That's a cross product of `c` and `a` productions.
        """
        grammar[from_nt] = (
            [r for r in grammar[from_nt]
                   if r[:1] != [to_nt]] +
            [c + a[1:] for c in grammar[to_nt]
                           for a in grammar[from_nt]
                               if a[:1] == [to_nt]]
        )

    def eliminate_immediate_left_recursion(nt):
        """Rewrite the productions of `nt` so that none start with the symbol `nt`.

        nt ::= nt a0 | ... | b0 | ...
        ==> nt ::= b0 nt' | ...
            nt' ::= (empty) | a0 nt' | ...
        """
        rules = grammar[nt]
        if any(r[:1] == [nt] for r in rules):
            epilogue = gensym(grammar, nt)
            grammar[epilogue] = [[]] + [r[1:] + [epilogue]
                                        for r in rules
                                            if r[:1] == [nt]]
            grammar[nt] = [r + [epilogue]
                           for r in rules
                               if r[:1] != [nt]]

    ntnames = quasisort_nts()
    for i, iname in enumerate(ntnames):
        for j, jname in enumerate(ntnames[:i]):
            eliminate_left_calls(from_nt=iname, to_nt=jname)
        eliminate_immediate_left_recursion(iname)


def clone_grammar(grammar):
    return {nt: [prod[:] for prod in prods] for nt, prods in grammar.items()}


def common_prefix(lists):
    """Return the longest sequence S such that every list in `lists` starts with S."""
    common = None
    for L in lists:
        if common is None:
            common = L[:]
        else:
            i = 0
            while i < len(common) and i < len(L) and common[i] == L[i]:
                i += 1
            del common[i:]
    assert common is not None
    return common


def left_factor(grammar):
    """ Left-factor the given non-left-recursive `grammar`. """
    grammar = clone_grammar(grammar)
    todo = list(grammar.items())
    while todo:
        nt, prods = todo.pop()

        silos = collections.defaultdict(list)
        for prod in prods:
            if len(prod) > 0 and not is_reduction(prod[0]):
                silos[prod[0]].append(prod)

        out = []
        for prod in prods:
            if len(prod) == 0 or is_reduction(prod[0]) or len(silos[prod[0]]) == 1:
                out.append(prod)
            elif len(silos[prod[0]]) > 1:
                assert prod[0] != nt  # should not be left-recursion
                factor_set = silos[prod[0]]
                common_seq = common_prefix(factor_set)
                assert len(common_seq) > 0
                tail_nt = gensym(grammar, nt)
                out.append(common_seq + [tail_nt])
                grammar[tail_nt] = []  # for gensym's benefit
                todo.append((tail_nt, [prod[len(common_seq):] for prod in factor_set]))
                del factor_set[:]  # don't do this one again later
        grammar[nt] = out

    return grammar


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


def check_ambiguity(grammar, goal):
    """Throw if the given grammar, which must already be non-left-recursive, isn't LL(1)."""
    follow = follow_sets(grammar, goal)
    for nt, prods in grammar.items():
        start = set()
        for prod in prods:
            prod_start = seq_start(grammar, prod)
            conflicts = prod_start & start
            if conflicts:
                # The grammar is not LL(1). It may not actually be ambiguous,
                # but this simplistic analysis can't prove it unambiguous.
                if conflicts == {EMPTY}:
                    # Definitely ambiguous.
                    raise ValueError("ambiguous grammar: multiple productions for {!r} "
                                     "match the empty string".format(nt))
                else:
                    conflicts -= {EMPTY}
                    raise ValueError("unsupported grammar: multiple productions for {!r} "
                                     "match strings that start with {!r}"
                                     .format(nt, list(conflicts)[0]))
            start |= prod_start

        # If nt can match the empty string, then we also have to check that
        # there is no ambiguity between matching the empty string and matching
        # a nonempty string. This is done by comparing the start set we've just
        # computed with nt's follow set. (If the grammar is left-recursive, this
        # step will error out, even though the grammar is not really ambiguous.)
        if EMPTY in start:
            conflicts = start & follow[nt]
            if conflicts:
                raise ValueError("unsupported grammar: the token {!r} could start either "
                                 "a string matching {!r} or something that follows it"
                                 .format(list(conflicts)[0], nt))


def generate_parser(out, grammar, goal):
    # First, append a natural reduction step at the end of every production.
    # This ensures that the parser we eventually generate builds parse trees
    # matching the *original* grammar, no matter how we transform the grammar
    # internally.
    grammar = {nt: [prod + [Reduction(nt, i, len(prod))]
                    for i, prod in enumerate(productions)]
               for nt, productions in grammar.items()}

    check(grammar)
    eliminate_left_recursion(grammar)
    grammar = left_factor(grammar)
    check_ambiguity(grammar, goal)
    follow = follow_sets(grammar, goal)

    # Build parse table.
    parse_table = {}
    for nt, prods in grammar.items():
        # Set of terminals that can be the first token of a match for any rule
        # we've considered so far. We track this to rule out ambiguity (overzealously).
        #
        # We track this set even when we're emitting code for left-recursive productions;
        # it's not necessary, because check() imposes a much tougher rule, but the extra
        # checking doesn't hurt anything.
        seen = set()
        empty_production = None
        for i, prod in enumerate(prods):
            start_set = seq_start(grammar, prod)
            if start_set & seen:
                raise ValueError("invalid grammar: ambiguous token(s) {}".format(start_set & seen))
            seen |= start_set
            if seen == {EMPTY}:
                assert empty_production is None
                empty_production = prod
            else:
                for token in start_set:
                    parse_table[nt, token] = prod[::-1]
        if empty_production is not None:
            for token in follow[nt]:
                parse_table[nt, token] = empty_production[::-1]

    # Write the parser.
    out.write("import pgen_runtime\n"
              "from pgen_runtime import Reduction\n\n")
    out.write("parse_table = {}\n\n".format(pprint.pformat(parse_table, width=99)))
    out.write("parse = pgen_runtime.make_parse_fn(parse_table, {!r})\n".format(goal))

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
