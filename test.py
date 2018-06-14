#!/usr/bin/env python3

import gen
import io, unittest
import re
import gen
from gen import Optional, LookaheadRule
import lexer


LispTokenizer = lexer.LexicalGrammar("( )", SYMBOL=r'[!%&*+:<=>?@A-Z^_a-z~]+')


class GenTestCase(unittest.TestCase):
    def compile(self, tokenize, grammar):
        """Compile a grammar. Use this when you expect compilation to succeed."""
        self.tokenize = tokenize
        self.parse = gen.compile(grammar, next(iter(grammar)))

    def assertParse(self, s):
        self.parse(self.tokenize(s))

    def assertNoParse(self, s, message="banana"):
        tokens = self.tokenize(s)
        self.assertRaisesRegex(SyntaxError, re.escape(message), lambda: self.parse(tokens))

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
        parse = gen.compile(grammar, "expr")

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

    def testEnd(self):
        self.compile(
            lexer.LexicalGrammar("ONE TWO"),
            {
                'goal': [
                    ['ONE', 'TWO']
                ]
            }
        )
        self.assertNoParse("ONE TWO TWO",
                           "expected 'end of input', got 'TWO'")

    def testList(self):
        list_grammar = {
            'prelist': [
                ['word', 'list']
            ],
            'list': [
                ['word'],
                ['list', 'word'],
            ],
            'word': [
                ['SYMBOL']
            ],
        }
        parse = gen.compile(list_grammar, "prelist")
        tokens = LispTokenizer("the quick brown fox jumped over the lazy dog")
        self.assertEqual(
            parse(tokens),
            ('prelist', 0, [
                ('word', 0, ['the']),
                ('list', 1, [
                    ('list', 1, [
                        ('list', 1, [
                            ('list', 1, [
                                ('list', 1, [
                                    ('list', 1, [
                                        ('list', 1, [
                                            ('list', 0, [('word', 0, ['quick'])]),
                                            ('word', 0, ['brown'])
                                        ]),
                                        ('word', 0, ['fox'])
                                    ]),
                                    ('word', 0, ['jumped'])
                                ]),
                                ('word', 0, ['over'])
                            ]),
                            ('word', 0, ['the'])
                        ]),
                        ('word', 0, ['lazy'])
                    ]),
                    ('word', 0, ['dog'])
                ])
            ])
        )

    def testArithmetic(self):
        tokenize = lexer.LexicalGrammar("+ - * / ( )", NUM=r'[0-9]\w*', VAR=r'[A-Za-z]\w*')
        arith_grammar = {
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
        }
        parse = gen.compile(arith_grammar, "expr")

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

        self.assertRaisesRegex(SyntaxError,
                               r"expected one of \['\(', 'NUM', 'VAR'], got None",
                               lambda: parse(tokenize("(")))
        self.assertRaisesRegex(SyntaxError,
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

    def testLeftFactor(self):
        """Most basic left-factoring test."""
        tokenize = lexer.LexicalGrammar("A B")
        grammar = {
            'goal': [
                ['A'],
                ['A', 'B'],
            ],
        }

        parse = gen.compile(grammar, goal='goal')
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
        parse = gen.compile(grammar, goal='goal')
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
        parse = gen.compile(grammar, goal='stmt')
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
        parse = gen.compile(grammar, 's')
        tokenize = lexer.LexicalGrammar("A B C")

        self.assertEqual(parse(tokenize("A B")),
                         ('s', 0, [('x', 0, ['A']), 'B']))
        self.assertEqual(parse(tokenize("A C")),
                         ('s', 1, [('y', 0, ['A']), 'C']))

    def testLeftHandSideExpression(self):
        """Example of a grammar that's in SLR(1) but hard to smoosh into an LL(1) form.

        This is taken from the ECMAScript grammar.

        ...Of course, it's not really possible to enforce the desired syntactic
        restrictions in LR(k) either; the ES grammar matches `(x + y) = z` and
        an additional attribute grammar (IsValidSimpleAssignmentTarget) is
        necessary to rule it out.
        """
        self.compile(
            lexer.LexicalGrammar("= +", VAR=r'[a-z]+\b'),
            {
                'AssignmentExpression': [
                    ['AdditiveExpression'],
                    ['LeftHandSideExpression', '=', 'AssignmentExpression'],
                ],
                'AdditiveExpression': [
                    ['LeftHandSideExpression'],
                    ['AdditiveExpression', '+', 'LeftHandSideExpression'],
                ],
                'LeftHandSideExpression': [
                    ['VAR'],
                ]
            }
        )
        self.assertParse("z = x + y")
        self.assertNoParse("x + y = z", "expected one of ['+', 'end of input'], got '='")

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
        parse = gen.compile(grammar, "expr")

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

    def testExpandOptional(self):
        self.assertEqual(
            list(gen.expand_optional_symbols(['ONE', 'TWO', '3'])),
            [(['ONE', 'TWO', '3'], [])])
        self.assertEqual(
            list(gen.expand_optional_symbols(['a', 'b', Optional('c')])),
            [(['a', 'b'], [2]),
             (['a', 'b', 'c'], [])])
        self.assertEqual(
            list(gen.expand_optional_symbols([Optional('a'), Optional('b')])),
            [([], [0, 1]),
             (['a'], [1]),
             (['b'], [0]),
             (['a', 'b'], [])])

    def testEmptyGrammar(self):
        tokenize = lexer.LexicalGrammar("X")
        self.compile(tokenize, {'goal': [[]]})
        self.assertEqual(
            self.parse(self.tokenize("")),
            ('goal', 0, [])
        )
        self.assertNoParse("X", "expected 'end of input', got 'X' (line 1)")

    def testOptionalEmpty(self):
        tokenize = lexer.LexicalGrammar("X Y")
        grammar = {
            'a': [
                [Optional('b'), Optional('c')],
            ],
            'b': [
                ['X'],
            ],
            'c': [
                ['Y'],
            ]
        }
        parse = gen.compile(grammar, "a")
        self.assertEqual(parse(tokenize("")), ('a', 0, [None, None]))
        self.assertEqual(parse(tokenize("X")), ('a', 0, [('b', 0, ['X']), None]))
        self.assertEqual(parse(tokenize("Y")), ('a', 0, [None, ('c', 0, ['Y'])]))
        self.assertEqual(parse(tokenize("X Y")), ('a', 0, [('b', 0, ['X']), ('c', 0, ['Y'])]))

    def testOptional(self):
        tokenize = lexer.LexicalGrammar('[ ] , X')
        grammar = {
            'array': [
                ['[', Optional('elision'), ']'],
                ['[', 'elements', ']'],
                ['[', 'elements', ',', Optional('elision'), ']']
            ],
            'elements': [
                [Optional('elision'), 'X'],
                ['elements', ',', Optional('elision'), 'X']
            ],
            'elision': [
                [','],
                ['elision', ',']
            ]
        }
        parse = gen.compile(grammar, 'array')
        self.assertEqual(parse(tokenize("[]")),
                         ('array', 0, ['[', None, ']']))
        self.assertEqual(parse(tokenize("[,]")),
                         ('array', 0, ['[',
                                       ('elision', 0, [',']),
                                       ']']))
        self.assertEqual(
            parse(tokenize("[,,X,,X,]")),
            ('array', 2, [
                '[',
                ('elements', 1, [
                    ('elements', 0, [
                        ('elision', 1, [
                            ('elision', 0, [',']),
                            ','
                        ]),
                        'X'
                    ]),
                    ',',
                    ('elision', 0, [',']),
                    'X'
                ]),
                ',',
                None,
                ']'
            ])
        )

    def testPositiveLookahead(self):
        self.compile(
            lexer.LexicalGrammar('A B + ( )'),
            {
                'goal': [
                    [LookaheadRule(frozenset({'A', 'B'}), True), 'expr'],
                ],
                'expr': [
                    ['term'],
                    ['expr', '+', 'term'],
                ],
                'term': [
                    ['A'],
                    ['B'],
                    ['(', 'expr', ')'],
                ]
            }
        )
        self.assertNoParse("(A)", "expected one of ['A', 'B'], got '('")
        self.assertParse("A + B")

    def testNegativeLookahead(self):
        tokenize = lexer.LexicalGrammar('a b')
        grammar = {
            'goal': [
                [LookaheadRule(frozenset({'a'}), False), 'abs'],
            ],
            'abs': [
                ['a'],
                ['b'],
                ['abs', 'a'],
                ['abs', 'b'],
            ],
        }

        parse = gen.compile(grammar, 'goal')
        self.assertRaisesRegex(SyntaxError,
                               r"expected 'b', got 'a'",
                               lambda: parse(tokenize("ab")))
        self.assertEqual(
            parse(tokenize('ba')),
            ('goal', 0, [
                ('abs', 2, [
                    ('abs', 1, ['b']),
                    'a'
                ]),
            ])
        )

        # In simple cases like this, the lookahead restriction can even
        # disambiguate a grammar that would otherwise be ambiguous.
        grammar['goal'].append(['a'])
        parse = gen.compile(grammar, 'goal')
        self.assertEqual(
            parse(tokenize('a')),
            ('goal', 1, ['a'])
        )

    def disabledNegativeLookaheadDisambiguation(self):
        tokenize = lexer.LexicalGrammar('( ) { } ; function =', IDENT=r'[A-Za-z_][A-Za-z_0-9]*')
        grammar = {
            'stmts': [
                ['stmt'],
                ['stmts', 'stmt'],
            ],
            'stmt': [
                [LookaheadRule(set=frozenset({'function'}), positive=False), 'expr', ';'],
                ['fndecl'],
            ],
            'fndecl': [
                ['function', 'IDENT', '(', ')', '{', Optional('stmt'), '}'],
            ],
            'expr': [
                ['term'],
                ['IDENT', '=', 'expr'],
            ],
            'term': [
                ['(', 'expr', ')'],
                ['fndecl'],
                ['term', '(', 'expr', ')'],
            ],
        }
        parse = gen.compile(grammar, 'stmts')

        # Test that without the lookahead restriction, we reject this grammar
        # (it's ambiguous):
        del grammar['stmt'][0][0]
        self.assertRaisesRegex(ValueError,
                               'banana',
                               lambda: gen.compile(grammar, 'stmt'))

        self.assertEqual(
            parse(tokenize('function f() { x = function y() {}; }')),
            ('stmts', 0, [
                ('stmt', 1, [
                    ('fndecl', 0, [
                        'function', 'f', '(', ')', '{',
                        ('stmt', 0, [
                            ('expr', 1, [
                                'x',
                                '=',
                                ('expr', 0, [
                                    ('term', 1, [
                                        ('fndecl', 0, [
                                            'function', 'y', '(', ')', '{', None, '}',
                                        ]),
                                    ]),
                                ]),
                            ]),
                            ';',
                        ]),
                    ]),
                ]),
            ])
        )

        self.assertEqual(
            parse(tokenize('(function g(){});')),
            ('stmts', 0, [
                ('stmt', 0, [
                    ('expr', 0, [
                        ('term', 1, [
                            ('fndecl', 0, [
                                'function', 'g', '(', ')', '{', None, '}',
                            ]),
                        ]),
                    ]),
                    ';',
                ]),
            ])
        )

    def testTrailingLookahead(self):
        """Lookahead at the end of a production is banned."""
        grammar = {
            'stmt': [
                ['OTHER', ';'],
                ['IF', '(', 'X', ')', 'stmt', LookaheadRule(frozenset({'ELSE'}), False)],
                ['IF', '(', 'X', ')', 'stmt', 'ELSE', 'stmt'],
            ],
        }
        self.assertRaisesRegex(ValueError,
                               r"invalid grammar: lookahead restriction at end of production",
                               lambda: gen.compile(grammar, 'stmt'))

    def testLookaheadBeforeOptional(self):
        self.compile(
            lexer.LexicalGrammar('= : _', PUBLIC=r'public\b', IDENT=r'[a-z]+\b', NUM=r'[0-9]\b'),
            {
                'decl': [
                    [LookaheadRule(frozenset({'IDENT'}), True), Optional('attrs'), 'pat', '=', 'NUM'],
                ],
                'attrs': [
                    ['attr'],
                    ['attrs', 'attr'],
                ],
                'attr': [
                    ['PUBLIC', ':'],
                    ['IDENT', ':'],
                ],
                'pat': [
                    ['IDENT'],
                    ['_'],
                ],
            }
        )
        self.assertParse("x = 0")
        self.assertParse("thread: x = 0")
        self.assertNoParse("public: x = 0", "expected 'IDENT', got 'PUBLIC'")
        self.assertNoParse("_ = 0", "expected 'IDENT', got '_'")
        self.assertParse("funny: public: x = 0")
        self.assertParse("funny: _ = 0")

    def testForLookahead(self):
        grammar = {
            'Stmt': [
                [';'],
                ['ForStmt'],
            ],
            'ForStmt': [
                ["for", "(", LookaheadRule(frozenset({"let"}), False),
                 "Expr", ";", ";", ")", "Stmt"],
            ],
            'Expr': [
                ["0"],
                ["let"],
            ],
        }
        self.compile(lexer.LexicalGrammar("for ( let ; ) 0"), grammar)
        self.assertParse("for (0;;) ;")
        self.assertNoParse("for (let;;) ;", "expected '0', got 'let'")

    # XXX to test: combination of lookaheads, ++, +-, -+, --
    # XXX todo: find an example where lookahead canonicalization matters



if __name__ == '__main__':
    unittest.main()
