""" Data structures for representing grammars. """

import collections
from ordered import OrderedSet, OrderedFrozenSet
import typing


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
    rules = {
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

    # Variable terminals are terminal symbols that can have several different
    # values, like a VAR token that could be any identifier, or a NUM token
    # that could be any number.
    variable_terminals = ['NUM', 'VAR']
    return Grammar(rules, variable_terminals)


# A Grammar object is just an object that contains a bunch of productions, like
# the example grammar above. Since we have no other way of distinguishing
# terminals from nonterminals, we store the set of terminals and the set of
# nonterminals in the Grammar.
class Grammar:
    def __init__(self, nonterminals, variable_terminals=()):

        # This constructor type-checks the arguments.
        #
        # This only checks types. It doesn't check that the grammar is LR, that
        # it's cycle-free, or any other nice properties.
        #
        # Normally, good Python style is never to check types but to plow ahead
        # and let the language throw if the caller has erred. Here, the values
        # are quite large, errors are likely, and if we don't check, the
        # eventual TypeError doesn't usefully point to the location of the
        # problem. So we check up front.
        #
        # (More justification: it's very sad to throw a bad error message while
        # building a good one or while debugging. Passing these checks means a
        # grammar can be used safely with `dump`, `rhs_to_str`, and so on.)

        if not isinstance(nonterminals, typing.Mapping):
            raise TypeError("expected nonterminals dict, not {!r}".format(type(nonterminals).__name__))

        self.variable_terminals = OrderedFrozenSet(variable_terminals)
        self.nonterminals = {}
        all_terminals = OrderedSet(self.variable_terminals)

        def validate_element(nt, i, e):
            # As a side effect, this populates all_terminals
            if isinstance(e, str):
                if e not in nonterminals:
                    all_terminals.add(e)
            elif isinstance(e, Optional):
                if not isinstance(e.inner, str):
                    raise TypeError("invalid grammar: unrecognized element in production `grammar[{!r}][{}].inner`: {!r}"
                                    .format(nt, i, e.inner))
                if e.inner not in nonterminals:
                    all_terminals.add(e.inner)
            elif not isinstance(e, (LookaheadRule, Apply)):
                raise TypeError("invalid grammar: unrecognized element in production `grammar[{!r}][{}]`: {!r}"
                                .format(nt, i, e))
            return e

        def copy_rhs(nt, i, rhs):
            if not isinstance(rhs, typing.Iterable):
                raise TypeError("invalid grammar: grammar[{!r}][{}] should be a list of grammar symbols, not {!r}"
                                .format(nt, i, type(rhs).__name__))
            return [validate_element(nt, i, e) for e in rhs]


        for nt, rhs_list_or_fn in nonterminals.items():
            if not isinstance(nt, str):
                raise TypeError("invalid grammar: expected string keys in nonterminals dict, got {!r}".format(nt))
            if nt in self.variable_terminals:
                raise TypeError("invalid grammar: {!r} is both a nonterminal and a variable terminal".format(nt))

            # A nonterminal maps to either a single function or (more typically) a
            # list of right-hand sides.  Function nonterminals can't be
            # type-checked here; we check the result after calling them.
            if callable(rhs_list_or_fn):
                self.nonterminals[nt] = rhs_list_or_fn
            else:
                self.nonterminals[nt] = [copy_rhs(nt, i, rhs) for i, rhs in enumerate(rhs_list_or_fn)]

        self.terminals = OrderedFrozenSet(all_terminals)

    def check_valid_goals(self, goal_nts):
        for goal in goal_nts:
            if not self.is_nt(goal):
                raise ValueError("goal nonterminal {!r} is undefined".format(goal))




    # Terminals are tokens that must appear verbatim in the input wherever they
    # appear in the grammar, like the operators '+' '-' *' '/' and brackets '(' ')'
    # in the example grammar.
    def is_terminal(self, element):
        return type(element) is str and element in self.terminals

    def is_variable_terminal(self, element):
        return type(element) is str and element in self.variable_terminals

    # Nonterminals refer to other rules.
    def is_nt(self, element):
        return type(element) is str and element in self.nonterminals

    def clone(self):
        """ Return a deep copy of a grammar (which must contain no functions). """
        return Grammar(self.nonterminals, self.variable_terminals)


    # === A few methods for dumping pieces of grammar.

    def element_to_str(self, e):
        if self.is_nt(e):
            return e
        elif is_apply(e):
            return "{}[{}]".format(e.nt, ", ".join(repr(arg) for arg in e.args))
        elif self.is_terminal(e):
            if self.is_variable_terminal(e):
                return e
            return '"' + repr(e)[1:-1] + '"'
        elif is_optional(e):
            return self.element_to_str(e.inner) + "?"
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

    def rhs_to_str(self, rhs):
        return " ".join(self.element_to_str(e) for e in rhs)

    def production_to_str(self, nt, rhs):
        return "{} ::= {}".format(nt, self.rhs_to_str(rhs))

    def lr_item_to_str(self, prods, item):
        prod = prods[item.prod_index]
        if item.lookahead is None:
            la = []
        else:
            la = [self.element_to_str(item.lookahead)]
        return "{} ::= {} >> {{{}}}".format(
            prod.nt,
            " ".join([self.element_to_str(e) for e in prod.rhs[:item.offset]]
                     + ["\N{MIDDLE DOT}"]
                     + la
                     + [self.element_to_str(e) for e in prod.rhs[item.offset:]]),
            ", ".join(
                "$" if t is None else self.element_to_str(t)
                for t in item.followed_by)
        )

    def item_set_to_str(self, prods, item_set):
        return "{{{}}}".format(
            ",  ".join(self.lr_item_to_str(prods, item) for item in item_set)
        )
 
    def dump(self):
        for nt, rhs_list in self.nonterminals.items():
            print(nt + " ::=")
            if callable(rhs_list):
                print("   ", repr(rhs_list))
            else:
                for rhs in rhs_list:
                    if rhs:
                        print("   ", self.rhs_to_str(rhs))
                    else:
                        print("   [empty]")
            print()


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
