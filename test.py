#!/usr/bin/env python3

import gen
import io, unittest
import re

class LexicalGrammar:
    def __init__(self, tokens, **regexps):
        token_list = sorted(tokens.split(), key=len, reverse=True)
        self.token_re = re.compile("|".join(re.escape(token) for token in token_list))
        self.parser_pairs = [(k, re.compile(v)) for k, v in regexps.items()]

    def __call__(self, source):
        return Tokenizer(self.token_re, self.parser_pairs, source)


class Tokenizer:
    def __init__(self, token_re, parser_pairs, source):
        self.token_re = token_re
        self.parser_pairs = parser_pairs
        self.src = source
        self.point = 0

    def _match(self):
        while self.point < len(self.src) and self.src[self.point] in " \t":
            self.point += 1
        if self.point == len(self.src):
            return None
        match = self.token_re.match(self.src, self.point)
        if match is not None:
            return match.group(0), match
        for name, pattern in self.parser_pairs:
            match = pattern.match(self.src, self.point)
            if match is not None:
                return name, match
        raise ValueError("unexpected characters {!r}".format(self.src[self.point:self.point+4]))

    def peek(self):
        hit = self._match()
        if hit is None:
            return None
        return hit[0]

    def take(self, k):
        hit = self._match()
        if hit is None:
            if k is not None:
                raise ValueError("unexpected end of input (expected {!r})".format(k))
        else:
            name, match = hit
            if k != name:
                raise ValueError("expected {!r}, got {!r}".format(k, name))
            self.point = match.end()
            return match.group()


def compile(grammar, goal='expr'):
    out = io.StringIO()
    gen.generate_parser(out, grammar, goal)
    scope = {}
    exec(out.getvalue(), scope)
    return scope['parse']


class GenTestCase(unittest.TestCase):
    def testSimple(self):
        tokenize = LexicalGrammar("( )", SYMBOL=r'[!%&*+:<=>?@A-Z^_a-z~]+')
        grammar = {
            'expr': [
                ['SYMBOL'],
                ['(', 'tail'],
            ],
            'tail': [
                [')'],
                ['expr', 'tail'],
            ],
        }
        parse = compile(grammar)

        parsed = parse(tokenize("(lambda (x) (* x x))"))
        self.assertEqual(parsed, ('expr', 1, [
            '(',
            ('tail', 1, [
                ('expr', 0, ['lambda']),
                ('tail', 1, [
                    ('expr', 1, ['(', ('tail', 1, [('expr', 0, ['x']), ('tail', 0, [')'])])]),
                    ('tail', 1, [
                        ('expr', 1, [
                            '(',
                            ('tail', 1, [
                                ('expr', 0, ['*']),
                                ('tail', 1, [
                                    ('expr', 0, ['x']),
                                    ('tail', 1, [('expr', 0, ['x']), ('tail', 0, [')'])])
                                ])
                            ])
                        ]),
                        ('tail', 0, [')'])
                    ])
                ])
            ])
        ]))

    def testArithmetic(self):
        tokenize = LexicalGrammar("+ - * / ( )", NUM=r'[0-9]\w*', VAR=r'[A-Za-z]\w*')
        parse = compile({
            'expr': [
                ['term'],
                ['expr', '+', 'term'],
                ['expr', '-', 'term'],
            ],
            'term': [
                ['prim'],
                ['term', '*', 'prim'],
                ['term', '/', 'prim'],
            ],
            'prim': [
                ['NUM'],
                ['VAR'],
                ['(', 'expr', ')'],
            ],
        })
        tokens = tokenize('2 * 3 + 4 * (5 + 7)')
        self.assertEqual(
            parse(tokens),
            ('expr', 1, [
                ('expr', 0, [
                    ('term', 1, [
                        ('term', 0, [('prim', 0, ['2'])]),
                        '*',
                        ('prim', 0, ['3'])
                    ])
                ]),
                '+',
                ('term', 1, [
                    ('term', 0, [('prim', 0, ['4'])]),
                    '*',
                    ('prim', 2, [
                        '(',
                        ('expr', 1, [
                            ('expr', 0, [('term', 0, [('prim', 0, ['5'])])]),
                            '+',
                            ('term', 0, [('prim', 0, ['7'])])
                        ]),
                        ')'
                    ])
                ])
            ])
        )

    def testAmbiguous(self):
        # This grammar should fail verification.
        # It's ambiguous: is ABC s(A)y(BC) or s(AB)y(C)?
        grammar = {
            'goal': [
                ['s', 'y'],
            ],
            's': [
                ['A'],
                ['s', 'B'],
            ],
            'y': [
                ['C'],
                ['B', 'C'],
            ],
        }

        out = io.StringIO()
        self.assertRaisesRegex(ValueError, r"unsupported grammar: the token 'B' could start either a string matching 's_' or something that follows it",
                               lambda: gen.generate_parser(out, grammar, 'goal'))

    def testUndefinedNt(self):
        grammar = {
            'goal': [
                ['expr'],
            ],
        }
        out = io.StringIO()
        self.assertRaisesRegex(ValueError, r"invalid grammar: nonterminal 'expr' is used but not defined",
                               lambda: gen.generate_parser(out, grammar, 'goal'))

    def testLeftFactor(self):
        tokenize = LexicalGrammar("A B")
        grammar = {
            'goal': [
                ['A'],
                ['A', 'B'],
            ],
        }

        parse = compile(grammar, goal='goal')
        self.assertEqual(parse(tokenize("A")), ('goal', 0, ['A']))
        self.assertEqual(parse(tokenize("A B")), ('goal', 1, ['A', 'B']))

    def testLeftFactorMulti(self):
        tokenize = LexicalGrammar("A B C D E")
        grammar = {
            'goal': [
                ['A', 'B', 'C', 'D'],
                ['A', 'B', 'C', 'E'],
            ],
        }
        parse = compile(grammar, goal='goal')
        self.assertEqual(parse(tokenize("A B C D")), ('goal', 0, ['A', 'B', 'C', 'D']))
        self.assertEqual(parse(tokenize("A B C E")), ('goal', 1, ['A', 'B', 'C', 'E']))


if __name__ == '__main__':
    unittest.main()
