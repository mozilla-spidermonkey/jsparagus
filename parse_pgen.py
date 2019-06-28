#!/usr/bin/env python

"""parse_pgen.py - Parse grammars written in the pgen parser specification language.

I'm not sure I want to keep this pgen mini-language around; ignore this for now.
"""

from lexer import LexicalGrammar
from grammar import Grammar, Optional
import gen
import pprint
import parse_pgen_generated
import unittest
from collections import namedtuple


pgen_lexer = LexicalGrammar("goal nt { } ; ?",
                            r'([ \t\r\n]|#.*)*',
                            IDENT=r'[A-Za-z_](?:\w|[_-])*',
                            STR=r'"[^\\\n"]*"')


pgen_grammar = Grammar(
    {
        'grammar': [[Optional('token_defs'), 'nt_defs']],
        'token_defs': [['token_def'], ['token_defs', 'token_def']],
        'token_def': [
            ['token', 'IDENT', '=', 'STR', ';'],
            ['var', 'token', 'IDENT', ';']
        ],
        'nt_defs': [['nt_def'], ['nt_defs', 'nt_def']],
        'nt_def': [
            ['nt', 'IDENT', '{', gen.Optional('prods'), '}'],
            ['goal', 'nt', 'IDENT', '{', gen.Optional('prods'), '}'],
        ],
        'prods': [['prod'], ['prods', 'prod']],
        'prod': [['terms', ';']],
        'terms': [['term'], ['terms', 'term']],
        'term': [['symbol'], ['symbol', '?']],
        'symbol': [['IDENT'], ['STR']],
    },
    ['IDENT', 'STR']
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
    def grammar_P0(self, token_defs, nt_defs):
        nonterminals, goal_nts = nt_defs
        return (token_defs or default_token_list, nonterminals, goal_nts)

    def token_defs_P0(self, token_def): return [token_def]
    def token_defs_P1(self, token_defs, token_def): return token_defs + [token_def]

    def token_def_P0(self, token, name, eq, picture, semi):
        assert (token, eq, semi) == ('token', '=', ';')
        return (name, picture)

    def token_def_P1(self, var, token, name, semi):
        assert (var, token, semi) == ('var', 'token', ';')
        return (name, None)

    def nt_defs_P0(self, nt_def): return self.nt_defs_P1(({}, []), nt_def)
    def nt_defs_P1(self, grammar_in, nt_def):
        is_goal, nt, prods = nt_def
        grammar, goal_nts = grammar_in
        if nt in grammar:
            raise ValueError("multiple definitions for nt {}".format(nt))
        grammar[nt] = prods
        if is_goal:
            goal_nts.append(nt)
        return grammar, goal_nts

    def nt_def_P0(self, *args):
        nt_kw, ident, lc, prods, rc = args
        assert (nt_kw, lc, rc) == ('nt', '{', '}')
        return (False, ident, prods)
    def nt_def_P1(self, *args):
        goal_kw, nt_kw, ident, lc, prods, rc = args
        assert (goal_kw, nt_kw, lc, rc) == ('goal', 'nt', '{', '}')
        return (True, ident, prods)

    def prods_P0(self, prod): return [prod]
    def prods_P1(self, prods, prod): return prods + [prod]

    def prod_P0(self, symbols, semi):
        assert semi == ';'
        return symbols

    def terms_P0(self, term): return [term]
    def terms_P1(self, terms, term): return terms + [term]

    def term_P0(self, sym): return sym
    def term_P1(self, sym, q):
        assert q == '?'
        return gen.Optional(sym)

    def symbol_P0(self, sym): return sym
    def symbol_P1(self, sym):
        assert len(sym) > 1
        assert sym[0] == '"'
        assert sym[-1] == '"'
        chars = sym[1:-1]  # This is a bit sloppy.
        return Literal(chars)

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
        for i, rhs in enumerate(rhs_list):
            out_rhs = []
            for e in rhs:
                e = check_element(nt, i, e)
                out_rhs.append(e)
            out[nt].append(out_rhs)

    return (tokens, out, goal_nts)

def load_grammar(filename):
    with open(filename) as f:
        text = f.read()
    builder = AstBuilder()
    result = parse_pgen_generated.parse_grammar(pgen_lexer(text, filename=filename), builder)
    tokens, nonterminals, goals = check_grammar(result)
    variable_terminals = [name for name, image in tokens if image is None]
    return Grammar(nonterminals, variable_terminals), goals


def regenerate():
    import sys
    gen.generate_parser(sys.stdout, pgen_grammar, ['grammar'])


class ParsePgenTestCase(unittest.TestCase):
    def test_self(self):
        import os
        filename = os.path.join(os.path.dirname(__file__), "pgen.pgen")
        grammar, goal_nts = load_grammar(filename)
        self.assertEqual(grammar.nonterminals, pgen_grammar.nonterminals)
        self.assertEqual(grammar.variable_terminals, pgen_grammar.variable_terminals)
        self.assertEqual(goal_nts, ['grammar'])

        import io
        out = io.StringIO()
        gen.generate_parser(out, grammar, ["grammar"])
        self_generated = out.getvalue()

        with open(parse_pgen_generated.__file__) as f:
            pre_generated = f.read()

        self.maxDiff = None
        self.assertEqual(self_generated, pre_generated)


if __name__ == '__main__':
    unittest.main()
