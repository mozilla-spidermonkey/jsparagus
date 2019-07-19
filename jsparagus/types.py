"""Type inference for reduce expressions.

A type is one of the following:

*   `UnitType`, which is kind of like `void` in Java or `None` in Python,
    a type that doesn't carry any information.

*   `'str'` or `'bool'` - That is, the Python strings, which stand for the
    target language's string type and boolean type respectively. 'str' is the
    type of variable terminals, and 'bool' is the type of optional
    non-variable terminals.

*   `NtType(name)` - A type. We generate at most one of these per
    nonterminal in the language.

*   `Option(t)`, an option type.

*   `TypeVar()`, a type variable that might be bound to any type.
    This is used only during inference.

In addition, MethodType simply gathers together a return type and a list of
argument types.

See infer_types() for more.
"""

import collections
from . import grammar


class JsparagusTypeError(Exception):
    def annotate(self, line):
        message, *rest = self.args
        message = line + "\n" + message
        self.args = message, *rest


def type_to_str(ty):
    ty = deref(ty)
    if ty is UnitType:
        return "()"
    elif isinstance(ty, str):
        return ty
    elif isinstance(ty, OptionType):
        return "Option<{}>".format(type_to_str(ty.t))
    elif isinstance(ty, TypeVar):
        if ty.name is not None:
            return ty.name
        else:
            return repr(ty)
    else:
        raise TypeError("not a type: {!r}".format(ty))


UnitType = ()


class OptionType:
    __slots__ = ['t']

    def __init__(self, t):
        self.t = t


NtType = collections.namedtuple("NtType", "name")


class TypeVar:
    """A type variable, used only during type inference.

    The point of type inference is to assign each method and each nonterminal a
    return type; we start by assigning each one a type variable and then do
    unification, a la Hindley-Milner.

    Each type variable may be given a str `name` and a positive int
    `precedence`. These are used at the end of type inference, if we still
    don't know a concrete type for this variable--which is often the case for
    nonterminals.

    The precedence is used when two type variables are unified, to choose the
    better name. (Nonterminal names should take precedence over method names.)
    Greater precedence numbers mean higher precedence.
    """
    __slots__ = ['name', 'precedence', 'value']

    def __init__(self, name=None, precedence=0):
        assert (precedence > 0) == (name is not None)
        self.name = name
        self.precedence = precedence
        self.value = None


def deref(t):
    if isinstance(t, TypeVar):
        if t.value is not None:
            t.value = deref(t.value)
            return t.value
    return t


def final_deref(ty):
    """Like deref(), but also replace any remaining unresolved type variables with
    NtTypes.
    """
    ty = deref(ty)
    if isinstance(ty, TypeVar):
        if ty.name is not None:
            # ty becomes an nt type.
            ty.value = NtType(ty.name)
            return ty.value
        else:
            raise Exception("internal error: no way to assign a type to variable")
    elif isinstance(ty, OptionType):
        return OptionType(final_deref(ty.t))
    else:
        return ty


def unify(actual, expected):
    actual = deref(actual)
    expected = deref(expected)
    if actual == '!' or expected == '!':
        pass
    elif actual is UnitType and expected is UnitType:
        pass
    elif isinstance(actual, OptionType) and isinstance(expected, OptionType):
        unify(actual.t, expected.t)
    elif isinstance(actual, str) and isinstance(expected, str):
        if actual != expected:
            raise JsparagusTypeError(
                "expected type {}, got type {}"
                .format(actual, expected))
    elif isinstance(expected, TypeVar):
        assert expected.value is None
        if actual is not expected:
            if (isinstance(actual, TypeVar) and
                    actual.precedence <= expected.precedence):
                actual.value = expected
            else:
                expected.value = actual
    elif isinstance(actual, TypeVar):
        assert actual.value is None
        if actual is not expected:
            actual.value = expected
    else:
        raise JsparagusTypeError(
            "expected type {}, got type {}"
            .format(type_to_str(actual), type_to_str(expected)))


class MethodType:
    __slots__ = ['argument_types', 'return_type']

    def __init__(self, argument_types, return_type):
        self.argument_types = argument_types
        self.return_type = return_type

    def resolve(self):
        return MethodType(
            [final_deref(t) for t in self.argument_types],
            final_deref(self.return_type))


def infer_types(g):
    """Assign a type to each nonterminal and each method in a grammar.

    The type system is pretty rigid. We don't have any polymorphism or union
    types. If two of a nonterminal's productions have different types, this
    will typically just unify the two types, which can result in mysterious
    output. If it can't do that (e.g. if one of the types is `str`) then it
    throws a JsparagusTypeError.
    """

    nt_types = {
        nt: TypeVar(nt, 2)
        for nt in g.nonterminals
        if not isinstance(nt, grammar.InitNt)
    }

    method_types = {}

    def element_type(e):
        if isinstance(e, str):
            if e in g.nonterminals:
                return nt_types[e]
            elif e in g.variable_terminals:
                return 'str'
            else:
                # constant terminal
                return UnitType
        elif grammar.is_optional(e):
            return OptionType(element_type(e.inner))
        elif grammar.is_apply(e):
            return nt_types[e.nt]
        else:
            assert False, "unexpected element type: {!r}".format(e)

    def expr_type(expr):
        if isinstance(expr, int):
            return concrete_element_types[expr]
        elif expr is None:
            return OptionType(TypeVar())
        elif isinstance(expr, grammar.Some):
            return OptionType(expr_type(expr.inner))
        elif isinstance(expr, grammar.CallMethod):
            arg_types = [expr_type(arg) for arg in expr.args]
            if expr.method in method_types:
                mtype = method_types[expr.method]
                if len(expr.args) != len(mtype.argument_types):
                    raise JsparagusTypeError(
                        "method {!r} is called with {} argument(s) and with {} argument(s)"
                        .format(expr.method, len(expr.args), len(mtype.argument_types)))
                for i, (actual_type, expected_type) in enumerate(
                        zip(arg_types, mtype.argument_types)):
                    try:
                        unify(actual_type, expected_type)
                    except JsparagusTypeError as exc:
                        exc.annotate(
                            "error passing {} as argument {} to method {!r}:"
                            .format(
                                grammar.expr_to_str(expr.args[i]),
                                i + 1,
                                expr.method))
                        raise
            else:
                # Use method name as fallback type name (but low
                # precedence--this should be unified with something better).
                name = expr.method
                if ' ' in name:
                    name = name.split(' ')[0]

                mtype = MethodType(arg_types, TypeVar(name, 1))
                method_types[expr.method] = mtype
            return mtype.return_type
        else:
            raise TypeError("unrecognized reduce expr: {!r}".format(expr))

    for nt, plist_or_fn in g.nonterminals.items():
        if isinstance(nt, grammar.InitNt):
            continue
        nt_type = nt_types[nt]
        if isinstance(plist_or_fn, grammar.Parameterized):
            plist = plist_or_fn.body
        else:
            plist = plist_or_fn
        for i, p in enumerate(plist):
            if isinstance(p, grammar.ConditionalRhs):
                p = p.rhs
            concrete_element_types = [
                element_type(e)
                for e in p.body
                if grammar.is_concrete_element(e)
            ]
            try:
                unify(expr_type(p.action), nt_type)
            except JsparagusTypeError as exc:
                exc.annotate(
                    "in nonterminal {!r}, production {}:"
                    .format(nt, i + 1))
                raise

    nt_types = {name: final_deref(ty) for name, ty in nt_types.items()}
    method_types = {name: mtype.resolve() for name, mtype in method_types.items()}
    return nt_types, method_types
