#!/usr/bin/env python

"""parse_pgen.py - Parse grammars written in the pgen parser specification language.

I'm not sure I want to keep this pgen mini-language around; ignore this for now.
"""

import sys
from collections import namedtuple

from .lexer import LexicalGrammar
from .grammar import Grammar, Production, CallMethod, is_concrete_element, Optional
from . import gen
from . import parse_pgen_generated


pgen_lexer = LexicalGrammar("goal nt var token { } ; ? = =>",
                            r'([ \t\r\n]|#.*)*',
                            IDENT=r'[A-Za-z_](?:\w|[_-])*',
                            STR=r'"[^\\\n"]*"')


def list_of(e):
    nt = e + 's'
    return [
        Production(nt, [e], CallMethod('single', (0,))),
        Production(nt, [nt, e], CallMethod('append', (0, 1))),
    ]


def call_method(name, body):
    nargs = sum(1 for e in body if is_concrete_element(e))
    return CallMethod(name, tuple(range(nargs)))


def prod(nt, body, action):
    return Production(nt, body, call_method(action, body))


pgen_grammar = Grammar(
    {
        'grammar': [[Optional('token_defs'), 'nt_defs']],
        'token_defs': list_of('token_def'),
        'token_def': [
            prod('token_def', ['token', 'IDENT', '=', 'STR', ';'], 'const_token'),
            prod('token_def', ['var', 'token', 'IDENT', ';'], 'var_token'),
        ],
        'nt_defs': [
            prod('nt_defs', ['nt_def'], 'nt_defs_single'),
            prod('nt_defs', ['nt_defs', 'nt_def'], 'nt_defs_append'),
        ],
        'nt_def': [
            [Optional('goal'), 'nt', 'IDENT', '{', gen.Optional('prods'), '}'],
        ],
        'prods': list_of('prod'),
        'prod': [['terms', gen.Optional('action'), ';']],
        'terms': list_of('term'),
        'term': [
            ['symbol'],
            prod('term', ['symbol', '?'], 'optional'),
        ],
        'symbol': [
            prod('symbol', ['IDENT'], 'ident'),
            prod('symbol', ['STR'], 'str'),
        ],
        'action': [['=>', 'IDENT']],
    },
    goal_nts=['grammar'],
    variable_terminals=['IDENT', 'STR']
)


Literal = namedtuple("Literal", "chars")

default_token_list = [
    ("Var", "var"),
    ("Token", "token"),
    ("Goal", "goal"),
    ("Nt", "nt"),
    ("IDENT", None),
    ("STR", None),
    ("OpenBrace", "{"),
    ("CloseBrace", "}"),
    ("OpenParenthesis", "("),
    ("CloseParenthesis", ")"),
    ("Colon", ":"),
    ("EqualSign", "="),
    ("Asterisk", "*"),
    ("PlusSign", "+"),
    ("MinusSign", "-"),
    ("Slash", "/"),
    ("Semicolon", ";"),
    ("QuestionMark", "?"),
    ("RightArrow", "->"),
    ("Comma", ","),
]


class AstBuilder:
    def grammar(self, token_defs, nt_defs):
        nonterminals, goal_nts = nt_defs
        return (token_defs or default_token_list, nonterminals, goal_nts)

    def single(self, value):
        return [value]

    def append(self, values, value):
        values.append(value)
        return values

    def const_token(self, token, name, eq, picture, semi):
        assert (token, eq, semi) == ('token', '=', ';')
        assert picture[0] == '"'
        assert picture[-1] == '"'
        return (name, picture[1:-1])

    def var_token(self, var, token, name, semi):
        assert (var, token, semi) == ('var', 'token', ';')
        return (name, None)

    def nt_defs_single(self, nt_def):
        return self.nt_defs_append(({}, []), nt_def)

    def nt_defs_append(self, grammar_in, nt_def):
        is_goal, nt, prods = nt_def
        grammar, goal_nts = grammar_in
        if nt in grammar:
            raise ValueError("multiple definitions for nt {}".format(nt))
        grammar[nt] = prods
        if is_goal:
            goal_nts.append(nt)
        return grammar, goal_nts

    def nt_def(self, goal_kw, nt_kw, ident, lc, prods, rc):
        is_goal = goal_kw == "goal"
        assert (nt_kw, lc, rc) == ('nt', '{', '}')
        prods = [Production(ident, body, action) for body, action in prods]
        return (is_goal, ident, prods)

    def prod(self, symbols, action, semi):
        assert semi == ';'
        if action is None:
            if sum(1 for e in symbols if is_concrete_element(e)) == 1:
                action = 0
            else:
                raise ValueError("action required for {!r}".format(symbols))
        else:
            assert isinstance(action, str)
            action = call_method(action, symbols)
        return (symbols, action)

    def optional(self, sym, q):
        assert q == '?'
        return gen.Optional(sym)

    def ident(self, sym):
        return sym

    def str(self, sym):
        assert len(sym) > 1
        assert sym[0] == '"'
        assert sym[-1] == '"'
        chars = sym[1:-1]  # This is a bit sloppy.
        return Literal(chars)

    def action(self, arrow, ident):
        assert arrow == '=>'
        return ident


def check_grammar(result):
    tokens, nonterminals, goal_nts = result
    tokens_by_name = {}
    tokens_by_image = {}
    for name, image in tokens:
        if name in tokens_by_name:
            raise ValueError("token `{}` redeclared".format(name))
        tokens_by_name[name] = image
        if image is not None and image in tokens_by_image:
            raise ValueError("multiple tokens look like \"{}\"".format(image))
        tokens_by_image[image] = name
        if name in nonterminals:
            raise ValueError("`{}` is declared as both a token and a nonterminal (pick one)".format(name))

    def check_element(nt, i, e):
        if isinstance(e, Optional):
            return Optional(check_element(nt, i, e.inner))
        elif isinstance(e, Literal):
            if e.chars not in tokens_by_image:
                raise ValueError("in {} production {}: undeclared token \"{}\"".format(nt, i, e.chars))
            return e.chars
        else:
            assert isinstance(e, str), e.__class__.__name__
            if e in nonterminals:
                return e
            elif e in tokens_by_name:
                image = tokens_by_name[e]
                if image is not None:
                    return image
                return e
            else:
                raise ValueError("in {} production {}: undeclared symbol {}".format(nt, i, e))

    out = {nt: [] for nt in nonterminals}
    for nt, rhs_list in nonterminals.items():
        for i, p in enumerate(rhs_list):
            out_rhs = [check_element(nt, i, e) for e in p.body]
            out[nt].append(p.with_body(out_rhs))

    return (tokens, out, goal_nts)


def load_grammar(filename):
    with open(filename) as f:
        text = f.read()
    builder = AstBuilder()
    result = parse_pgen_generated.parse_grammar(pgen_lexer, text, builder)
    tokens, nonterminals, goals = check_grammar(result)
    variable_terminals = [name for name, image in tokens if image is None]
    return Grammar(nonterminals, goals, variable_terminals)


def regenerate():
    import sys
    gen.generate_parser(sys.stdout, pgen_grammar)


if __name__ == '__main__':
    if sys.argv[1:] == ['--regenerate']:
        regenerate()
    else:
        print("usage: ./parse_pgen.py --renegerate")
        sys.exit(1)
