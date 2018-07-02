#!/usr/bin/env python

"""parse_pgen.py - Parse grammars written in the pgen parser specification language.

I'm not sure I want to keep pgen around; ignore this for now.
"""

from lexer import LexicalGrammar
import gen
import pprint
import parse_pgen_generated
import unittest

pgen_lexer = LexicalGrammar("nt { } ; ?",
                            r'([ \t\r\n]|#.*)*',
                            IDENT=r'[A-Za-z_](?:\w|[_-])*',
                            STR=r'"[^\\\n"]*"')

pgen_grammar = {
    'grammar': [['nt_def'], ['grammar', 'nt_def']],
    'nt_def': [['nt', 'IDENT', '{', gen.Optional('prods'), '}']],
    'prods': [['prod'], ['prods', 'prod']],
    'prod': [['terms', ';']],
    'terms': [['term'], ['terms', 'term']],
    'term': [['symbol'], ['symbol', '?']],
    'symbol': [['IDENT'], ['STR']],
}

class AstBuilder:
    def __init__(self):
        self.identifiers_used = set()
        self.quoted_terminals_used = set()

    def grammar_0(self, nt_def): return self.grammar_1({}, nt_def)
    def grammar_1(self, grammar, nt_def):
        nt, prods = nt_def
        if nt in grammar:
            raise ValueError("multiple definitions for nt {}".format(nt))
        grammar[nt] = prods
        return grammar

    def nt_def_0(self, nt_kw, ident, lc, prods, rc):
        assert (nt_kw, lc, rc) == ('nt', '{', '}')
        return (ident, prods)

    def prods_0(self, prod): return [prod]
    def prods_1(self, prods, prod): return prods + [prod]

    def prod_0(self, symbols, semi):
        assert semi == ';'
        return symbols

    def terms_0(self, term): return [term]
    def terms_1(self, terms, term): return terms + [term]

    def term_0(self, sym): return sym
    def term_1(self, sym, q):
        assert q == '?'
        return gen.Optional(sym)

    def symbol_0(self, sym):
        self.identifiers_used.add(sym)
        return sym
    def symbol_1(self, sym):
        assert len(sym) > 1
        assert sym[0] == '"'
        assert sym[-1] == '"'
        chars = sym[1:-1]  # This is very sloppy.
        self.quoted_terminals_used.add(sym)
        return chars

    def check(self, grammar):
        for t in self.quoted_terminals_used:
            if t in self.identifiers_used:
                if t in grammar:
                    raise ValueError("nonterminal `{}` is also used as a quoted terminal "
                                     "(sorry, they're not allowed to look the same; rename the nonterminal)"
                                     .format(t))
                else:
                    raise ValueError("nonterminal `{}` is used both quoted and nonquoted; pick one".format(t))
        for t in self.identifiers_used:
            if t not in grammar:
                if not all(c.isupper() or c == '_' for c in t):
                    print("Warning: symbol `{}` is not defined as a nonterminal in the grammar "
                          "(if it's a terminal, you can silence this warning by renaming it to SHOUTY_CASE)"
                          .format(t))

depth=0

def postparse(builder, cst):
    global depth
    if isinstance(cst, tuple) and len(cst) == 3 and (isinstance(cst[0], str)
                                                     and isinstance(cst[1], int)
                                                     and isinstance(cst[2], list)):
        method_name = cst[0] + "_" + str(cst[1])
        method = getattr(builder, method_name, None)
        if method is not None:
            depth += 1
            args = [postparse(builder, kid) for kid in cst[2]]
            depth -= 1
            return method(*args)
    return cst


def load_grammar(filename):
    with open(filename) as f:
        text = f.read()
    result = parse_pgen_generated.parse(pgen_lexer(text, filename=filename))
    builder = AstBuilder()
    grammar = postparse(builder, result)
    builder.check(grammar)
    return grammar


def regenerate():
    import sys
    gen.generate_parser(sys.stdout, pgen_grammar, 'grammar')


class ParsePgenTestCase(unittest.TestCase):
    def test_self(self):
        import os
        filename = os.path.join(os.path.dirname(__file__), "pgen.pgen")
        grammar = load_grammar(filename)
        self.assertEqual(grammar, pgen_grammar)

        import io
        out = io.StringIO()
        gen.generate_parser(out, grammar, "grammar")
        self_generated = out.getvalue()

        with open(parse_pgen_generated.__file__) as f:
            pre_generated = f.read()

        self.assertEqual(self_generated, pre_generated)


if __name__ == '__main__':
    unittest.main()
