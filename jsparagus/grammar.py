""" Data structures for representing grammars. """

import collections
from .ordered import OrderedSet, OrderedFrozenSet
from . import types


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

    # The goal nonterminals are the nonterminals we're actually interested in
    # parsing. Here we want to parse expressions; all the other nonterminals
    # are interesting only as the building blocks of expressions.
    #
    # Variable terminals are terminal symbols that can have several different
    # values, like a VAR token that could be any identifier, or a NUM token
    # that could be any number.
    return Grammar(rules, goal_nts=['expr'], variable_terminals=['NUM', 'VAR'])


# A production consists of a left side, an optional condition, a right side,
# and a reduce action.  A `Production` object includes everything except the
# left side.  Incorporating actions lets us transform grammar while preserving
# behavior.
#
# The production `expr ::= term` is represented by
# `Production(["term"], 0)`.
#
# The production `expr ::= expr + term => add` is represented by
# `Production(["expr", "+", "term"], CallMethod("add", (0, 1, 2))`.
#
class Production:
    __slots__ = ['body', 'action', 'condition']

    def __init__(self, body, action, *, condition=None):
        self.body = body
        self.action = action
        self.condition = condition

    def __eq__(self, other):
        return (self.body == other.body
                and self.action == other.action
                and self.condition == other.condition)

    __hash__ = None

    def __repr__(self):
        if self.condition is None:
            return "Production({!r}, action={!r})".format(self.body, self.action)
        else:
            return ("Production({!r}, action={!r}, condition={!r})"
                    .format(self.body, self.action, self.condition))

    def copy_with(self, **kwargs):
        return Production(body=kwargs.get('body', self.body),
                          action=kwargs.get('action', self.action),
                          condition=kwargs.get('condition', self.condition))


# ### Reduce actions
#
# Reduce actions say what happens when a production is matched.
#
# Reduce expressions are a little language used to specify reduce
# actions. There are two types of reduce expression:
#
# *   An integer in the range(0, len(production.body)) returns a previously parsed
#     value from the parser's stack.
#
# *   CallMethod objects pass values to a builder method and return the result.
#     The `args` are nested reduce expressions.
#
# *   None is an expression used as a placeholder when an optional symbol is
#     omitted.
#
# *   Some(expr) is used when an optional symbol is found and parsed.
#     In Python, this just expands to the same thing as `expr`, but in Rust
#     this expands to a use of `Option::Some()`.
#
# In addition, the special reduce action 'accept' means stop parsing. This is
# used only in productions for init nonterminals, created automatically by
# Grammar.__init__(). It's not a reduce expression, so it can't be nested.
#
CallMethod = collections.namedtuple("CallMethod", "method args")
Some = collections.namedtuple("Some", "inner")


def expr_to_str(expr):
    if isinstance(expr, int):
        return "${}".format(expr)
    elif isinstance(expr, CallMethod):
        return "{}({})".format(
            expr.method,
            ', '.join(expr_to_str(arg) for arg in expr.args))
    elif expr is None:
        return "None"
    elif isinstance(expr, Some):
        return "Some({})".format(expr_to_str(expr.inner))
    elif expr == "accept":
        return "<accept>"
    else:
        raise ValueError("unrecognized expression: {!r}".format(expr))


# A Grammar object is just an object that contains a bunch of productions, like
# the example grammar above. Since we have no other way of distinguishing
# terminals from nonterminals, we store the set of terminals and the set of
# nonterminals in the Grammar.
class Grammar:
    def __init__(
            self,
            nonterminals,
            goal_nts=None,
            variable_terminals=(),
            type_info=None):

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

        if goal_nts is None:
            # Default to the first nonterminal in the dictionary.
            goal_nts = []
            for name in nonterminals:
                goal_nts.append(name)
                break
        else:
            goal_nts = list(goal_nts)

        self.variable_terminals = OrderedFrozenSet(variable_terminals)
        self.nonterminals = {nt: None for nt in nonterminals}

        # Validate, desugar, and copy the grammar. As a side effect, calling
        # validate_element on every element of the grammar populates
        # all_terminals.
        all_terminals = OrderedSet(self.variable_terminals)

        def validate_element(nt, i, j, e, context_params):
            if isinstance(e, str):
                if e not in nonterminals:
                    all_terminals.add(e)
                return e
            elif isinstance(e, Optional):
                if not isinstance(e.inner, (str, Apply)):
                    raise TypeError(
                        "invalid grammar: unrecognized element "
                        "in production `grammar[{!r}][{}][{}].inner`: {!r}"
                        .format(nt, i, j, e.inner))
                validate_element(nt, i, j, e.inner, context_params)
                return e
            elif isinstance(e, Apply):
                # Either the application or the original parameterized
                # production must be present in the dictionary.
                if e not in nonterminals and e.name not in nonterminals:
                    raise ValueError(
                        "invalid grammar: unrecognized nonterminal "
                        "in production `grammar[{!r}][{}][{}]`: {!r}"
                        .format(nt, i, j, e.name))
                args = [pair[0] for pair in e.args]
                if (e.name in nonterminals
                        and args != list(nonterminals[e.name].params)):
                    raise ValueError(
                        "invalid grammar: wrong arguments passed to {!r} "
                        "in production `grammar[{!r}][{}][{}]`: "
                        "passed {!r}, expected {!r}"
                        .format(e.name, nt, i, j, e.name,
                                args, list(nonterminals[e.name].params)))
                for param_name, arg_expr in e.args:
                    if isinstance(arg_expr, Var):
                        if arg_expr.name not in context_params:
                            raise ValueError(
                                "invalid grammar: undefined variable {!r} "
                                "in production `grammar[{!r}][{}][{}]`"
                                .format(arg_expr.name, nt, i, j))
                return e
            elif isinstance(e, LookaheadRule) or e is ErrorToken:
                return e
            else:
                raise TypeError(
                    "invalid grammar: unrecognized element in production "
                    "`grammar[{!r}][{}][{}]`: {!r}"
                    .format(nt, i, j, e))
            return e

        def check_reduce_action(nt, i, rhs, action):
            if isinstance(action, int):
                concrete_len = sum(1 for e in rhs.body
                                   if is_concrete_element(e))
                if not (0 <= action < concrete_len):
                    raise ValueError(
                        "invalid grammar: element number {} out of range for "
                        "production {!r} in grammar[{!r}][{}].action ({!r})"
                        .format(action, nt, rhs.body, i, rhs.action))
            elif isinstance(action, CallMethod):
                if not isinstance(action.method, str):
                    raise TypeError(
                        "invalid grammar: method names must be strings, "
                        "not {!r}, in grammar[{!r}[{}].action"
                        .format(action.method, nt, i))
                if not action.method.isidentifier():
                    name, space, pn = action.method.partition(' ')
                    if space == ' ' and name.isidentifier() and pn.isdigit():
                        pass
                    else:
                        raise ValueError(
                            "invalid grammar: invalid method name {!r} "
                            "(not an identifier), in grammar[{!r}[{}].action"
                            .format(action.method, nt, i))
                for arg_expr in action.args:
                    check_reduce_action(nt, i, rhs, arg_expr)
            elif action is None:
                pass
            elif isinstance(action, Some):
                check_reduce_action(nt, i, rhs, action.inner)
            else:
                raise TypeError(
                    "invalid grammar: unrecognized reduce expression {!r} "
                    "in grammar[{!r}][{}].action"
                    .format(action, nt, i))

        def copy_rhs(nt, i, sole_production, rhs, context_params):
            if isinstance(rhs, Production):
                if rhs.condition is not None:
                    param, value = rhs.condition
                    if param not in context_params:
                        raise TypeError(
                            "invalid grammar: undefined parameter {!r} "
                            "in conditional for grammar[{!r}][{}]"
                            .format(param, nt, i))
                if rhs.action != 'accept':
                    check_reduce_action(nt, i, rhs, rhs.action)
                assert isinstance(rhs.body, list)
                return rhs.copy_with(body=[
                    validate_element(nt, i, j, e, context_params)
                    for j, e in enumerate(rhs.body)
                ])
            elif isinstance(rhs, list):
                # Bare list, no action. Desugar to a Production, inferring a
                # reasonable default action.
                nargs = sum(1 for e in rhs if is_concrete_element(e))
                if len(rhs) == 1 and nargs == 1:
                    action = 0  # don't call a method, just propagate the value
                else:
                    # Call a method named after the production. If the
                    # nonterminal has exactly one production, there's no need
                    # to include the production index `i` to the method name.
                    if sole_production:
                        method = nt
                    else:
                        method = '{} {}'.format(nt, i)
                    action = CallMethod(method, args=tuple(range(nargs)))
                return copy_rhs(nt, i, sole_production, Production(rhs, action), context_params)
            else:
                raise TypeError(
                    "invalid grammar: grammar[{!r}][{}] should be "
                    "a list of grammar symbols, not {!r}"
                    .format(nt, i, rhs))

        def copy_nt_def(nt, nt_def, params):
            if isinstance(nt_def, NtDef):
                for i, param in enumerate(nt_def.params):
                    if not isinstance(param, str):
                        raise TypeError(
                            "invalid grammar: parameter {} of {} should be "
                            "a string, not {!r}"
                            .format(i + 1, nt, param))
                params = nt_def.params[:]
                rhs_list = nt_def.rhs_list
            else:
                params = []
                rhs_list = nt_def

            if not isinstance(rhs_list, list):
                raise TypeError(
                    "invalid grammar: grammar[{!r}] should be either a "
                    "list of right-hand sides or NtDef, not {!r}"
                    .format(nt, type(rhs_list).__name__))

            sole_production = len(rhs_list) == 1
            rhs_list = [copy_rhs(nt, i, sole_production, rhs, params)
                        for i, rhs in enumerate(rhs_list)]
            return NtDef(params, rhs_list)

        def validate_nt(nt, nt_def):
            if isinstance(nt, InitNt):
                # Users don't include init nonterminals when initially creating
                # a Grammar. They are automatically added below. But if this
                # Grammar is being created by hacking on a previous Grammar, it
                # will already have them.
                if not isinstance(nt.goal, str):
                    raise TypeError(
                        "invalid grammar: InitNt.goal should be a string, "
                        "got {!r}"
                        .format(nt))
                if nt.goal not in nonterminals:
                    raise TypeError(
                        "invalid grammar: undefined nonterminal referenced "
                        "by InitNt: {!r}"
                        .format(nt))
                if nt.goal not in goal_nts:
                    raise TypeError(
                        "invalid grammar: nonterminal referenced by InitNt "
                        "is not in the list of goals: {!r}"
                        .format(nt))
                # Check the form of init productions. Initially these look like
                # [[goal]], but after the pipeline goes to work, they can be
                # [[Optional(goal)]] or [[], [goal]].
                if isinstance(nt_def, NtDef):
                    rhs_list = nt_def.rhs_list
                else:
                    rhs_list = nt_def
                if (rhs_list != [Production([nt.goal], 'accept')]
                        and rhs_list != [Production([Optional(nt.goal)], 'accept')]
                        and rhs_list != [Production([], 'accept'),
                                         Production([nt.goal], 'accept')]):
                    raise ValueError(
                        "invalid grammar: grammar[{!r}] is not one of "
                        "the expected forms: got {!r}"
                        .format(nt, rhs_list))
            elif isinstance(nt, Apply):
                if not isinstance(nt.name, str) or not isinstance(nt.args, tuple):
                    raise TypeError(
                        "invalid grammar: expected str or Apply(name=str, "
                        "args=tuple) keys in nonterminals dict, got {!r}"
                        .format(nt))
                for pair in nt.args:
                    if (not isinstance(pair, tuple)
                            or len(pair) != 2
                            or not isinstance(pair[0], str)
                            or not isinstance(pair[1], bool)):
                        raise TypeError(
                            "invalid grammar: expected tuple((str, bool)) args, got {!r}"
                            .format(nt))
            elif isinstance(nt, str):
                if not nt.isidentifier():
                    raise ValueError(
                        "invalid grammar: nonterminal names must be identifiers, not {!r}"
                        .format(nt))
            else:
                raise TypeError(
                    "invalid grammar: expected string keys in nonterminals dict, got {!r}"
                    .format(nt))
            if nt in self.variable_terminals:
                raise TypeError(
                    "invalid grammar: {!r} is both a nonterminal and a variable terminal"
                    .format(nt))
            return copy_nt_def(nt, nt_def, [])

        for nt, nt_def in nonterminals.items():
            self.nonterminals[nt] = validate_nt(nt, nt_def)

        # Cache the set of terminals for is_terminal.
        self.terminals = OrderedFrozenSet(all_terminals)

        # Check types of reduce expressions and infer method types. But if the
        # caller passed in precalculated type info, skip it -- otherwise we
        # would redo type checking many times as we make minor changes to the
        # Grammar along the pipeline.
        if type_info is None:
            type_info = types.infer_types(self)
        self.nt_types, self.methods = type_info

        # Synthesize "init" nonterminals.
        self.init_nts = []
        for goal in goal_nts:
            if goal not in nonterminals:
                raise ValueError(
                    "goal nonterminal {!r} is undefined".format(goal))
            init_nt = InitNt(goal)
            if init_nt not in self.nonterminals:
                self.nonterminals[init_nt] = NtDef(
                    [], [Production([goal], 'accept')])
            self.init_nts.append(init_nt)

    # Terminals are tokens that must appear verbatim in the input wherever they
    # appear in the grammar, like the operators '+' '-' *' '/' and brackets '(' ')'
    # in the example grammar.
    def is_terminal(self, element):
        return type(element) is str and element in self.terminals

    def is_variable_terminal(self, element):
        return type(element) is str and element in self.variable_terminals

    # Nonterminals refer to other rules.
    def is_nt(self, element):
        return isinstance(element, (str, Apply)) and element in self.nonterminals

    def goals(self):
        """Return a list of this grammar's goal nonterminals."""
        return [init_nt.goal for init_nt in self.init_nts]

    def clone(self):
        """Return a deep copy of a grammar (which must contain no functions)."""
        return Grammar(self.nonterminals, self.goals(), self.variable_terminals)

    def with_nonterminals(self, nonterminals):
        """Return a copy of self with the same attributes except different nonterminals."""
        return Grammar(
            nonterminals, self.goals(), self.variable_terminals,
            (self.nt_types, self.methods))

    # === A few methods for dumping pieces of grammar.

    def element_to_str(self, e):
        if isinstance(e, Apply):
            def arg_to_str(name, value):
                if value is True:
                    return '+' + name
                elif value is False:
                    return '~' + name
                elif isinstance(value, Var):
                    if value.name == name:
                        return '?' + value.name
                    return name + "=" + value.name
                else:
                    return name + "=" + repr(value)

            return "{}[{}]".format(e.name,
                                   ", ".join(arg_to_str(name, value)
                                             for name, value in e.args))
        elif self.is_nt(e):
            return e
        elif self.is_terminal(e):
            if self.is_variable_terminal(e):
                return e
            return '"' + repr(e)[1:-1] + '"'
        elif isinstance(e, Optional):
            return self.element_to_str(e.inner) + "?"
        elif isinstance(e, LookaheadRule):
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
        if isinstance(rhs, Production):
            if rhs.condition is None:
                prefix = ''
            else:
                param, value = rhs.condition
                if value is True:
                    condition = "+" + param
                elif value is False:
                    condition = "~" + param
                else:
                    condition = "{} == {!r}".format(param, value)
                prefix = "#[if {}] ".format(condition)
            return prefix + self.rhs_to_str(rhs.body)
        elif isinstance(rhs, Production):
            return self.rhs_to_str(rhs.body)
        elif len(rhs) == 0:
            return "[empty]"
        else:
            return self.symbols_to_str(rhs)

    def production_to_str(self, nt, rhs, action=()):
        # As we have two ways of representing productions at the moment, just
        # take multiple arguments :(
        return "{} ::= {}{}".format(
            self.element_to_str(nt),
            self.rhs_to_str(rhs),
            "" if action == () else " => " + expr_to_str(action))

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
        for nt, nt_def in self.nonterminals.items():
            left_side = self.element_to_str(nt)
            if nt_def.params:
                left_side += "[" + ", ".join(nt_def.params) + "]"
            print(left_side + " ::=")
            for rhs in nt_def.rhs_list:
                print("   ", self.rhs_to_str(rhs))
            print()


InitNt = collections.namedtuple("InitNt", "goal")
InitNt.__doc__ = """\
InitNt(goal) is the name of the init nonterminal for the given goal.

One init nonterminal is created internally for each goal symbol in the grammar.

The idea is to have a nonterminal that the user has no control over, that is
never used in any production, but *only* as an entry point for the grammar,
that always has a single production "init_nt ::= goal_nt". This predictable
structure makes it easier to get into and out of parsing at run time.

When an init nonterminal is matched, we take the "accept" action rather than
a "reduce" action.
"""


# *** Elements ****************************************************************
#
# Elements are the things that can appear in the .body list of a Production:
#
# *   Strings represent terminals and nonterminals (see `Grammar.is_nt`,
#     `Grammar.is_terminal`)
#
# *   `Apply` objects refer to parameterized nonterminals.
#
# *   `Optional` objects represent optional elements.
#
# *   `LookaheadRule` objects are like lookahead assertions in regular
#     expressions.
#
# *   The singleton object `ErrorToken` counts as a nonterminal but is never
#     produced by the lexer. Instead it is artificially injected into the token
#     stream just before a token that does not match anything.


def is_concrete_element(e):
    """True if parsing the element `e` pushes a value to the parser stack."""
    return (not isinstance(e, LookaheadRule)
            and e is not ErrorToken)


# Function application. Function nonterminals are expanded out very early in
# the process, before states are calculated, so most of the algorithm doesn't
# see these. They're replaced with gensym names.
Apply = collections.namedtuple("Apply", "name args")
Apply.__doc__ = """\
Apply(name, ((param0, arg0), ...)) is a call to a nonterminal function.

Nonterminals are like lambdas. Each nonterminal in a grammar is defined by an
NtDef which has 0 or more parameters.

To refer to a nonterminal that has 0 parameters, just use the nonterminal's
name. To use a parameterized nonterminal, we have to represent a function call
somehow; for that, use Apply.

Parameter names are strings. The arguments are typically booleans. They can be
whatever you want, but each function nonterminal gets expanded into a set of
productions, one for every different argument tuple that is ever passed to it.
"""


# Optional elements. These are expanded out before states are calculated,
# so the core of the algorithm never sees them.
Optional = collections.namedtuple("Optional", "inner")
Optional.__doc__ = """Optional(nt) matches either nothing or the given nt."""


# Lookahead restrictions stay with us throughout the algorithm.
LookaheadRule = collections.namedtuple("LookaheadRule", "set positive")
LookaheadRule.__doc__ = """\
LookaheadRule(set, pos) imposes a lookahead restriction on whatever follows.

It never consumes any tokens itself. Instead, the right-hand side
[LookaheadRule(frozenset(['a', 'b']), False), 'Thing']
matches a Thing that does not start with the token `a` or `b`.
"""


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


class ErrorTokenClass:
    """Special token that can be consumed to handle a syntax error."""

    def __new__(cls):
        global ErrorToken
        if ErrorToken is None:
            ErrorToken = object.__new__(ErrorTokenClass)
        return ErrorToken

    def __str__(self):
        return 'ErrorToken'

    def __repr__(self):
        # Note: If you change this, you're likely to break Python output, since
        # emit.py uses repr() in emitting parser tables.
        return 'ErrorToken'


ErrorToken = None
ErrorToken = ErrorTokenClass()


class NtDef:
    """Definition of a nonterminal.

    Instances have two attributes:

    .params - List of strings, the names of the parameters.

    .rhs_list - List of Production objects. Arguments to Apply elements in the
    productions can be Var(s) where `s in params`, indicating that parameter
    should be passed through unchanged.

    An NtDef is a sort of lambda.

    Some langauges have constructs that are allowed or disallowed in particular
    situations. For example, in many languages `return` statements are allowed
    only inside functions or methods. The ECMAScript standard (5.1.5 "Grammar
    Notation") offers this example of the notation it uses to specify this sort
    of thing:

        StatementList [Return] :
            [+Return] ReturnStatement
            ExpressionStatement

    This is an abbreviation for:

        StatementList :
            ExpressionStatement

        StatementList_Return :
            ReturnStatement
            ExpressionStatement

    We offer NtDef.params as a way of representing this in our system.

        "StatementList": NtDef(["Return"], [
            Production(["ReturnStatement"], condition=("Return", True)),
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

    """

    __slots__ = ['params', 'rhs_list']

    def __init__(self, params, rhs_list):
        self.params = params
        self.rhs_list = rhs_list

    def __eq__(self, other):
        return (isinstance(other, NtDef)
                and (self.params, self.rhs_list) == (other.params, other.rhs_list))

    __hash__ = None


Var = collections.namedtuple("Var", "name")
Var.__doc__ = """\
Var(name) represents the run-time value of the parameter with the given name.
"""
