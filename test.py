#!/usr/bin/env python3

import gen
import io, unittest
import re
import lexer

LispTokenizer = lexer.LexicalGrammar("( )", SYMBOL=r'[!%&*+:<=>?@A-Z^_a-z~]+')


def compile(grammar, goal='expr'):
    out = io.StringIO()
    gen.generate_parser(out, grammar, goal)
    scope = {}
    exec(out.getvalue(), scope)
    return scope['parse']


class GenTestCase(unittest.TestCase):
    def testSimple(self):
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

        parsed = parse(LispTokenizer("(lambda (x) (* x x))"))
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
        tokenize = lexer.LexicalGrammar("+ - * / ( )", NUM=r'[0-9]\w*', VAR=r'[A-Za-z]\w*')
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

        self.assertRaisesRegex(ValueError,
                               r"expected one of \['\(', 'NUM', 'VAR'], got None",
                               lambda: parse(tokenize("(")))
        self.assertRaisesRegex(ValueError,
                               r"expected one of \['\(', 'NUM', 'VAR'], got '\)'",
                               lambda: parse(tokenize(")")))

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
        self.assertRaisesRegex(ValueError, r"shift-reduce conflict",
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
        """Most basic left-factoring test."""
        tokenize = lexer.LexicalGrammar("A B")
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
        """Test left-factoring of grammars where some rules have a common prefix of length >1."""
        tokenize = lexer.LexicalGrammar("A B C D E")
        grammar = {
            'goal': [
                ['A', 'B', 'C', 'D'],
                ['A', 'B', 'C', 'E'],
            ],
        }
        parse = compile(grammar, goal='goal')
        self.assertEqual(parse(tokenize("A B C D")), ('goal', 0, ['A', 'B', 'C', 'D']))
        self.assertEqual(parse(tokenize("A B C E")), ('goal', 1, ['A', 'B', 'C', 'E']))

    def testLeftFactorMultiLevel(self):
        """Test left-factoring again on a nonterminal introduced by left-factoring."""
        tokenize = lexer.LexicalGrammar("FOR IN TO BY ( ) = ;", VAR=r'[A-Za-z]+')

        # The first left-factoring pass on `stmt` will left-factor `FOR ( VAR`.
        # A second pass is needed to left-factor `= expr TO expr`.
        grammar = {
            'stmt': [
                ['expr', ';'],
                ['FOR', '(', 'VAR', 'IN', 'expr', ')', 'stmt'],
                ['FOR', '(', 'VAR', '=', 'expr', 'TO', 'expr', ')', 'stmt'],
                ['FOR', '(', 'VAR', '=', 'expr', 'TO', 'expr', 'BY', 'expr', ')', 'stmt'],
                ['IF', '(', 'expr', ')', 'stmt'],
            ],
            'expr': [
                ['VAR'],
            ],
        }
        parse = compile(grammar, goal='stmt')
        self.assertEqual(parse(tokenize("FOR (x IN y) z;")),
                         ('stmt', 1, [
                             'FOR', '(', 'x', 'IN', ('expr', 0, ['y']), ')', ('stmt', 0, [
                                 ('expr', 0, ['z']), ';'
                             ])
                         ]))
        self.assertEqual(parse(tokenize("FOR (x = y TO z) x;")),
                         ('stmt', 2, [
                             'FOR', '(', 'x', '=', ('expr', 0, ['y']),
                             'TO', ('expr', 0, ['z']), ')', ('stmt', 0, [
                                 ('expr', 0, ['x']), ';'
                             ])
                         ]))
        self.assertEqual(parse(tokenize("FOR (x = y TO z BY w) x;")),
                         ('stmt', 3, [
                             'FOR', '(', 'x', '=', ('expr', 0, ['y']),
                             'TO', ('expr', 0, ['z']),
                             'BY', ('expr', 0, ['w']), ')', ('stmt', 0, [
                                 ('expr', 0, ['x']), ';'
                             ])
                         ]))

    def testFirstFirstConflict(self):
        """This grammar is unambiguous, but is not LL(1) due to a first/first conflict.

        Cribbed from: https://stackoverflow.com/a/17047370/94977
        """

        grammar = {
            's': [
                ['x', 'B'],
                ['y', 'C'],
            ],
            'x': [
                ['A'],
            ],
            'y': [
                ['A'],
            ],
        }
        parse = compile(grammar, 's')
        tokenize = lexer.LexicalGrammar("A B C")

        self.assertEqual(parse(tokenize("A B")),
                         ('s', 0, [('x', 0, ['A']), 'B']))
        self.assertEqual(parse(tokenize("A C")),
                         ('s', 1, [('y', 0, ['A']), 'C']))

    def testDeepRecursion(self):
        grammar = {
            'expr': [
                ['SYMBOL'],
                ['(', ')'],
                ['(', 'exprs', ')'],
            ],
            'exprs': [
                ['expr'],
                ['exprs', 'expr'],
            ],
        }
        parse = compile(grammar)

        N = 3000
        s = "x"
        t = ('expr', 0, ['x'])
        for i in range(N):
            s = "(" + s + ")"
            t = ('expr', 2, ['(', ('exprs', 0, [t]), ')'])

        result = parse(LispTokenizer(s))

        # Python can't check that result == t; it causes a RecursionError.
        # Testing that repr(result) == repr(t), same deal. So:
        for i in range(N):
            self.assertIsInstance(result, tuple)
            self.assertEqual(len(result), 3)
            self.assertEqual(result[0], 'expr')
            self.assertEqual(result[1], 2)
            result = result[2]
            self.assertIsInstance(result, list)
            self.assertEqual(len(result), 3)
            self.assertEqual(result[0], '(')
            self.assertEqual(result[2], ')')
            result = result[1]
            self.assertIsInstance(result, tuple)
            self.assertEqual(len(result), 3)
            self.assertEqual(result[0], 'exprs')
            self.assertEqual(result[1], 0)
            result = result[2]
            self.assertIsInstance(result, list)
            self.assertEqual(len(result), 1)
            result = result[0]


if __name__ == '__main__':
    unittest.main()
