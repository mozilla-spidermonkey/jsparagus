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

        nonterminals = dict(nonterminals.items())
        self.variable_terminals = OrderedFrozenSet(variable_terminals)
        self.nonterminals = {nt: None for nt in nonterminals}
        all_terminals = OrderedSet(self.variable_terminals)

        def validate_element(nt, i, j, e, context_params):
            # As a side effect, this populates all_terminals
            if isinstance(e, str):
                if e not in nonterminals:
                    all_terminals.add(e)
                return e
            elif isinstance(e, Optional):
                if not isinstance(e.inner, (str, Apply)):
                    raise TypeError("invalid grammar: unrecognized element in production `grammar[{!r}][{}][{}].inner`: {!r}"
                                    .format(nt, i, j, e.inner))
                validate_element(nt, i, j, e.inner, context_params)
                return e
            elif isinstance(e, Apply):
                if e.nt not in nonterminals:
                    raise ValueError("invalid grammar: unrecognized nonterminal in production `grammar[{!r}][{}][{}]`: {!r}"
                                    .format(nt, i, j, e.nt))
                args = [pair[0] for pair in e.args]
                if args != list(nonterminals[e.nt].params):
                    raise ValueError("invalid grammar: wrong arguments passed to {!r} in production `grammar[{!r}][{}][{}]`: passed {!r}, expected {!r}"
                                    .format(e.nt, nt, i, j, e.nt, args, list(nonterminals[e.nt].params)))
                for param_name, arg_expr in e.args:
                    if isinstance(arg_expr, Var):
                        if arg_expr.name not in context_params:
                            raise ValueError("invalid grammar: undefined variable {!r} in production `grammar[{!r}][{}][{}]`"
                                             .format(arg_expr.name, nt, i, j))
                return e
            elif isinstance(e, LookaheadRule):
                return e
            else:
                raise TypeError("invalid grammar: unrecognized element in production `grammar[{!r}][{}][{}]`: {!r}"
                                .format(nt, i, j, e))
            return e

        def copy_rhs(nt, i, rhs, context_params):
            #print("#COPY_RHS", rhs)
            if isinstance(rhs, ConditionalRhs):
                if rhs.param not in context_params:
                    raise TypeError("invalid grammar: undefined parameter {!r} in conditional for grammar[{!r}][{}]"
                                    .format(rhs.param, nt, i))
                return ConditionalRhs(rhs.param, rhs.value, copy_rhs(nt, i, rhs.rhs, context_params))
            elif isinstance(rhs, list):
                return [validate_element(nt, i, j, e, context_params) for j, e in enumerate(rhs)]
            else:
                raise TypeError("invalid grammar: grammar[{!r}][{}] should be a list of grammar symbols, not {!r}"
                                .format(nt, i, rhs))

        def copy_rhs_list(nt, rhs_list, params):
            if isinstance(rhs_list, Parameterized):
                fn = rhs_list
                params = list(fn.params)
                for i, param in enumerate(params):
                    if not isinstance(param, str):
                        raise TypeError("invalid grammar: parameter {} of {} should be a string, not {!r}"
                                        .format(i + 1, nt, param))
                assert isinstance(fn.body, list)
                return Parameterized(params, copy_rhs_list(nt, fn.body, params))
            else:
                if not isinstance(rhs_list, list):
                    raise TypeError("invalid grammar: grammar[{!r}] should be either a Parameterized object or a list of right-hand sides, not {!r}"
                                    .format(nt, type(rhs_list).__name__))
                return [copy_rhs(nt, i, rhs, params) for i, rhs in enumerate(rhs_list)]

        for nt, rhs_list_or_fn in nonterminals.items():
            if not isinstance(nt, str):
                raise TypeError("invalid grammar: expected string keys in nonterminals dict, got {!r}".format(nt))
            if nt in self.variable_terminals:
                raise TypeError("invalid grammar: {!r} is both a nonterminal and a variable terminal".format(nt))
            self.nonterminals[nt] = copy_rhs_list(nt, rhs_list_or_fn, [])

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
            def arg_to_str(name, value):
                if value == True:
                    return '+' + name
                elif value == False:
                    return '~' + name
                elif isinstance(value, Var):
                    if value.name == name:
                        return '?' + value.name
                    return name + "=" + value.name
                else:
                    return name + "=" + repr(value)

            return "{}[{}]".format(e.nt, ", ".join(arg_to_str(name, value)
                                                   for name, value in e.args))
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

    def symbols_to_str(self, rhs):
        return " ".join(self.element_to_str(e) for e in rhs)

    def rhs_to_str(self, rhs):
        if isinstance(rhs, ConditionalRhs):
            if rhs.value == True:
                condition = "+" + rhs.param
            elif rhs.value == False:
                condition = "~" + rhs.param
            else:
                condition = "{} == {!r}".format(rhs.param, rhs.value)
            return "#[if {}] ".format(condition) + self.rhs_to_str(rhs.rhs)
        elif len(rhs) == 0:
            return "[empty]"
        else:
            return self.symbols_to_str(rhs)

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
            if isinstance(rhs_list, Parameterized):
                nt += "[" + ", ".join(rhs_list.params) + "]"
                rhs_list = rhs_list.body
            print(nt + " ::=")
            for rhs in rhs_list:
                print("   ", self.rhs_to_str(rhs))
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
Apply(nt, ((param0, arg0), ...)) is a call to a nonterminal that's a function.

Each nonterminal in a grammar is defined by either a list of lists (its
productions) or a Parameterized, a lambda that returns a list of lists.

To refer to the first kind of nonterminal in a right-hand-side, just use the
nonterminal's name. To use the second kind, we have to represent a function call
somehow; for that, use Apply.

Parameter names are strings. The arguments are typically booleans. They can be
whatever you want, but each function nonterminal gets expanded into a set of
productions, one for every different argument tuple that is ever passed to it.
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


Parameterized = collections.namedtuple("Parameterized", "params body")
Parameterized.__doc__ = """\
Parameterized(params, rhs_list) - Lambda for nonterminals.

Some langauges have constructs that are allowed or disallowed in particular
situations. For example, in many languages `return` statements are allowed only
inside functions or methods. The ECMAScript standard (5.1.5 "Grammar Notation")
offers this example of the notation it uses to specify this sort of thing:

    StatementList [Return] :
        [+Return] ReturnStatement
        ExpressionStatement

This is an abbreviation for:

    StatementList :
        ExpressionStatement

    StatementList_Return :
        ReturnStatement
        ExpressionStatement

We offer Parameterized as a way of representing this in our system.

    "StatementList": Parameterized(["Return"], [
        Conditional("Return", True, ["ReturnStatement"]),
        ["ExpressionStatement"],
    ]),

This is an abbreviation for:

    "StatementList_0": [
        ["ExpressionStatement"],
    ],
    "StatementList_1": [
        ["ReturnStatement"],
        ["ExpressionStatement"],
    ],

Fields:

params - List of strings, the names of the parameters.

body - List of right-hand sides. Each element of rhs_list is either a list
of grammar elements or a ConditionalRhs (see below). Also, arguments to Apply
elements in the productions in rhs_list can be Var(s) where s in params,
indicating that parameter should be passed through unchanged.
"""


ConditionalRhs = collections.namedtuple("ConditionalRhs", "param value rhs")
ConditionalRhs.__doc__ = """\
ConditionalRhs(param, value, rhs) is just like rhs but conditionally ignored.
"""


Var = collections.namedtuple("Var", "name")
Var.__doc__ = """\
Var(name) represents the run-time value of the parameter with the given name.
"""

