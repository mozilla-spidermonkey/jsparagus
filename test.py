#!/usr/bin/env python3

import gen
import io, unittest
import re
import gen
from gen import Grammar, Production, CallMethod, Apply, Optional, LookaheadRule, Parameterized, ConditionalRhs, Var
import lexer


LispTokenizer = lexer.LexicalGrammar("( )", SYMBOL=r'[!%&*+:<=>?@A-Z^_a-z~]+')


def prod(nt, body, method_name):
    return Production(nt, body, CallMethod(method_name, list(range(len(body)))))


class GenTestCase(unittest.TestCase):
    def compile(self, tokenize, grammar):
        """Compile a grammar. Use this when you expect compilation to succeed."""
        self.tokenize = tokenize
        self.parse = gen.compile(grammar)

    def compile_multi(self, tokenize, grammar):
        self.tokenize = tokenize
        obj = gen.compile_multi(grammar)
        for attr in dir(obj):
            if attr.startswith("parse_"):
                setattr(self, attr, getattr(obj, attr))

    def assertParse(self, s, goal=None):
        method = "parse" if goal is None else "parse_" + goal
        getattr(self, method)(self.tokenize(s))

    def assertNoParse(self, s, *, goal=None, message="banana"):
        tokens = self.tokenize(s)
        method = "parse" if goal is None else "parse_" + goal
        self.assertRaisesRegex(SyntaxError, re.escape(message), lambda: getattr(self, method)(tokens))

    def testSimple(self):
        grammar = Grammar({
            'expr': [
                ['SYMBOL'],
                ['(', 'tail'],
            ],
            'tail': [
                [')'],
                ['expr', 'tail'],
            ],
        })
        parse = gen.compile(grammar)

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
            Grammar({
                'goal': [
                    ['ONE', 'TWO']
                ]
            })
        )
        self.assertNoParse("ONE TWO TWO",
                           message="expected 'end of input', got 'TWO'")

    def testList(self):
        list_grammar = Grammar({
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
        })
        parse = gen.compile(list_grammar)
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
        arith_grammar = Grammar({
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
        parse = gen.compile(arith_grammar)

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
        grammar = Grammar({
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
        })

        out = io.StringIO()
        self.assertRaisesRegex(ValueError, r"shift-reduce conflict",
                               lambda: gen.generate_parser(out, grammar))

    def testLeftFactor(self):
        """Most basic left-factoring test."""
        tokenize = lexer.LexicalGrammar("A B")
        grammar = Grammar({
            'goal': [
                ['A'],
                ['A', 'B'],
            ],
        })

        parse = gen.compile(grammar)
        self.assertEqual(parse(tokenize("A")), ('goal', 0, ['A']))
        self.assertEqual(parse(tokenize("A B")), ('goal', 1, ['A', 'B']))

    def testLeftFactorMulti(self):
        """Test left-factoring of grammars where some rules have a common prefix of length >1."""
        tokenize = lexer.LexicalGrammar("A B C D E")
        grammar = Grammar({
            'goal': [
                ['A', 'B', 'C', 'D'],
                ['A', 'B', 'C', 'E'],
            ],
        })
        parse = gen.compile(grammar)
        self.assertEqual(parse(tokenize("A B C D")), ('goal', 0, ['A', 'B', 'C', 'D']))
        self.assertEqual(parse(tokenize("A B C E")), ('goal', 1, ['A', 'B', 'C', 'E']))

    def testLeftFactorMultiLevel(self):
        """Test left-factoring again on a nonterminal introduced by left-factoring."""
        tokenize = lexer.LexicalGrammar("FOR IN TO BY ( ) = ;", VAR=r'[A-Za-z]+')

        # The first left-factoring pass on `stmt` will left-factor `FOR ( VAR`.
        # A second pass is needed to left-factor `= expr TO expr`.
        grammar = Grammar({
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
        })
        parse = gen.compile(grammar)
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

        grammar = Grammar({
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
        })
        parse = gen.compile(grammar)
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
            Grammar({
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
            })
        )
        self.assertParse("z = x + y")
        self.assertNoParse("x + y = z",
                           message="expected one of ['+', 'end of input'], got '='")

    def testDeepRecursion(self):
        grammar = Grammar({
            'expr': [
                ['SYMBOL'],
                ['(', ')'],
                ['(', 'exprs', ')'],
            ],
            'exprs': [
                ['expr'],
                ['exprs', 'expr'],
            ],
        })
        parse = gen.compile(grammar)

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
            list(gen.expand_optional_symbols_in_rhs(['ONE', 'TWO', '3'])),
            [(['ONE', 'TWO', '3'], [])])
        self.assertEqual(
            list(gen.expand_optional_symbols_in_rhs(['a', 'b', Optional('c')])),
            [(['a', 'b'], [2]),
             (['a', 'b', 'c'], [])])
        self.assertEqual(
            list(gen.expand_optional_symbols_in_rhs([Optional('a'), Optional('b')])),
            [([], [0, 1]),
             (['a'], [1]),
             (['b'], [0]),
             (['a', 'b'], [])])

    def testEmptyGrammar(self):
        tokenize = lexer.LexicalGrammar("X")
        self.compile(tokenize, Grammar({'goal': [[]]}))
        self.assertEqual(
            self.parse(self.tokenize("")),
            ('goal', 0, [])
        )
        self.assertNoParse("X", message="expected 'end of input', got 'X' (line 1)")

    def testOptionalEmpty(self):
        tokenize = lexer.LexicalGrammar("X Y")
        grammar = Grammar({
            'a': [
                [Optional('b'), Optional('c')],
            ],
            'b': [
                ['X'],
            ],
            'c': [
                ['Y'],
            ]
        })
        parse = gen.compile(grammar)
        self.assertEqual(parse(tokenize("")), ('a', 0, [None, None]))
        self.assertEqual(parse(tokenize("X")), ('a', 0, [('b', 0, ['X']), None]))
        self.assertEqual(parse(tokenize("Y")), ('a', 0, [None, ('c', 0, ['Y'])]))
        self.assertEqual(parse(tokenize("X Y")), ('a', 0, [('b', 0, ['X']), ('c', 0, ['Y'])]))

    def testOptional(self):
        tokenize = lexer.LexicalGrammar('[ ] , X')
        grammar = Grammar({
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
        })
        parse = gen.compile(grammar)
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
            Grammar({
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
            })
        )
        self.assertNoParse("(A)", message="expected one of ['A', 'B'], got '('")
        self.assertParse("A + B")

    def testNegativeLookahead(self):
        tokenize = lexer.LexicalGrammar('a b')
        rules = {
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

        parse = gen.compile(Grammar(rules))
        self.assertRaisesRegex(SyntaxError,
                               r"expected 'b', got 'a'",
                               lambda: parse(tokenize("a b")))
        self.assertEqual(
            parse(tokenize('b a')),
            ('goal', 0, [
                ('abs', 2, [
                    ('abs', 1, ['b']),
                    'a'
                ]),
            ])
        )

        # In simple cases like this, the lookahead restriction can even
        # disambiguate a grammar that would otherwise be ambiguous.
        rules['goal'].append(['a'])
        parse = gen.compile(Grammar(rules))
        self.assertEqual(
            parse(tokenize('a')),
            ('goal', 1, ['a'])
        )

    def disabledNegativeLookaheadDisambiguation(self):
        tokenize = lexer.LexicalGrammar('( ) { } ; function =', IDENT=r'[A-Za-z_][A-Za-z_0-9]*')
        grammar = Grammar({
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
        })
        parse = gen.compile(grammar)

        # Test that without the lookahead restriction, we reject this grammar
        # (it's ambiguous):
        del grammar['stmt'][0][0]
        self.assertRaisesRegex(ValueError,
                               'banana',
                               lambda: gen.compile(grammar))

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
        grammar = gen.Grammar({
            'stmt': [
                ['OTHER', ';'],
                ['IF', '(', 'X', ')', 'stmt', LookaheadRule(frozenset({'ELSE'}), False)],
                ['IF', '(', 'X', ')', 'stmt', 'ELSE', 'stmt'],
            ],
        })
        self.assertRaisesRegex(ValueError,
                               r"invalid grammar: lookahead restriction at end of production",
                               lambda: gen.compile(grammar))

    def testLookaheadBeforeOptional(self):
        self.compile(
            lexer.LexicalGrammar('= : _', PUBLIC=r'public\b', IDENT=r'[a-z]+\b', NUM=r'[0-9]\b'),
            Grammar({
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
            })
        )
        self.assertParse("x = 0")
        self.assertParse("thread: x = 0")
        self.assertNoParse("public: x = 0", message="expected 'IDENT', got 'PUBLIC'")
        self.assertNoParse("_ = 0", message="expected 'IDENT', got '_'")
        self.assertParse("funny: public: x = 0")
        self.assertParse("funny: _ = 0")

    def testForLookahead(self):
        grammar = Grammar({
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
        })
        self.compile(lexer.LexicalGrammar("for ( let ; ) 0"), grammar)
        self.assertParse("for (0;;) ;")
        self.assertNoParse("for (let;;) ;", message="expected '0', got 'let'")

    # XXX to test: combination of lookaheads, ++, +-, -+, --
    # XXX todo: find an example where lookahead canonicalization matters

    def testHugeExample(self):
        grammar = Grammar({
         'grammar': [['nt_def_or_blank_line'], ['grammar', 'nt_def_or_blank_line']],
         'arg': [['sigil', 'NT']],
         'args': [['arg'], ['args', ',', 'arg']],
         'definite_sigil': [['~'], ['+']],
         'exclusion': [['terminal'], ['nonterminal'], ['CHR', 'through', 'CHR']],
         'exclusion_list': [['exclusion'], ['exclusion_list', 'or', 'exclusion']],
         'ifdef': [['[', 'definite_sigil', 'NT', ']']],
         'line_terminator': [['NT'], ['NTALT']],
         'lookahead_assertion': [['==', 'terminal'],
                                 ['!=', 'terminal'],
                                 ['<!', 'NT'],
                                 ['<!', '{', 'lookahead_exclusions', '}']],
         'lookahead_exclusion': [['lookahead_exclusion_element'],
                                 ['lookahead_exclusion',
                                  'lookahead_exclusion_element']],
         'lookahead_exclusion_element': [['terminal'], ['no_line_terminator_here']],
         'lookahead_exclusions': [['lookahead_exclusion'],
                                  ['lookahead_exclusions', ',', 'lookahead_exclusion']],
         'no_line_terminator_here': [['[', 'no', 'line_terminator', 'here', ']']],
         'nonterminal': [['NT'], ['NTCALL', '[', 'args', ']']],
         'nt_def': [['nt_lhs', 'EQ', 'NL', 'rhs_lines', 'NL'],
                    ['nt_lhs', 'EQ', 'one', 'of', 'NL', 't_list_lines', 'NL']],
         'nt_def_or_blank_line': [['NL'], ['nt_def']],
         'nt_lhs': [['NT'], ['NTCALL', '[', 'params', ']']],
         'param': [['NT']],
         'params': [['param'], ['params', ',', 'param']],
         'rhs': [['symbols'], ['[', 'empty', ']']],
         'rhs_line': [[Optional(inner='ifdef'), 'rhs', Optional(inner='PRODID'), 'NL'],
                      ['PROSE', 'NL']],
         'rhs_lines': [['rhs_line'], ['rhs_lines', 'rhs_line']],
         'sigil': [['definite_sigil'], ['?']],
         'symbol': [['terminal'],
                    ['nonterminal'],
                    ['nonterminal', '?'],
                    ['nonterminal', 'but', 'not', 'exclusion'],
                    ['nonterminal', 'but', 'not', 'one', 'of', 'exclusion_list'],
                    ['[', 'lookahead', 'lookahead_assertion', ']'],
                    ['no_line_terminator_here'],
                    ['WPROSE']],
         'symbols': [['symbol'], ['symbols', 'symbol']],
         't_list_line': [['terminal_seq', 'NL']],
         't_list_lines': [['t_list_line'], ['t_list_lines', 't_list_line']],
         'terminal': [['T'], ['CHR']],
         'terminal_seq': [['terminal'], ['terminal_seq', 'terminal']]},
        variable_terminals='EQ T CHR NTCALL NT NTALT PRODID PROSE WPROSE'.split())

        emu_grammar_lexer = lexer.LexicalGrammar(
            #   the operators and keywords:
            "[ ] { } , ~ + ? <! == != but empty here lookahead no not of one or through",
            NL="\n",
            EQ=r':+',                           # any number of colons together
            T=r'`[^` \n]+`|```',                # terminals of the ES grammar, quoted with backticks
            CHR=r'<[A-Z]+>|U\+[0-9A-f]{4}',     # also terminals, denoting control characters
            NTCALL=r'(?:uri|[A-Z])\w*(?=\[)',   # nonterminals that will be followed by boolean parameters
            NT=r'(?:uri|[A-Z])\w*',             # nonterminals (also, boolean parameters)
            NTALT=r'\|[A-Z]\w+\|',              # nonterminals wrapped in vertical bars for no apparent reason
            PRODID=r'#[A-Za-z]\w*',             # the spec also gives a few productions names
            PROSE=r'>.*',                       # prose to the end of the line
            WPROSE=r'\[>[^]]*\]'                # prose wrapped in square brackets
            )

        self.compile(emu_grammar_lexer, grammar)

        source = """\
        IdentifierReference[Yield, Await] :
          Identifier
          [~Yield] `yield`
          [~Await] `await`

        """

        self.assertParse(source)

    def testParameterizedProductions(self):
        passthru = ('Yield', Var('Yield')),
        name = Apply("name", passthru)
        stmt = Apply("stmt", passthru)
        stmts = Apply("stmts", passthru)
        grammar = Grammar({
            'script': [
                ['def'],
                ['script', 'def'],
            ],
            'def': [
                ['function', 'IDENT', '(', ')', '{', Apply('stmts', (('Yield', False),)), '}'],
                ['function', '*', 'IDENT', '(', ')', '{', Apply('stmts', (('Yield', True),)), '}'],
            ],
            'stmts': Parameterized(['Yield'], [
                [stmt],
                [stmts, stmt],
            ]),
            'stmt': Parameterized(['Yield'], [
                [name, "(", ")", ";"],
                [name, "=", name, ";"],
                ConditionalRhs('Yield', True, ["yield", name, ";"]),
            ]),
            'name': Parameterized(['Yield'], [
                ["IDENT"],
                ConditionalRhs('Yield', False, ["yield"]),
            ]),
        }, variable_terminals=["IDENT"])
        self.compile(lexer.LexicalGrammar("( ) { } ; * = function yield",
                                          IDENT=r'[A-Za-z]\w*'),
                     grammar)
        self.assertParse("function* farm() { cow = pig; yield cow; }")
        self.assertNoParse("function city() { yield toOncomingTraffic; }",
                           message="expected one of ['(', ';', '='], got 'IDENT'")
        self.assertNoParse("function* farm() { yield = corn; yield yield; }",
                           message="expected 'IDENT', got '='")

    def testCanonicalLR(self):
        """Example 4.39 (grammar 4.20) from the book."""

        # Modified as marked below
        grammar = Grammar({
            "S": [
                ["L", "=", "R"],
                ["R"],
            ],
            "L": [
                ["*", "R"],
                ["id"],
            ],
            "R": [
                ["L"],
                ["7"], # added so we can have a negative test, showing R = R is not an S
            ],
        })
        self.compile(lexer.LexicalGrammar("id = * 7"), grammar)
        self.assertParse("id = *id")
        self.assertParse("*id = id")
        self.assertParse("id = 7")
        self.assertNoParse("7 = id", message="expected 'end of input', got '='")

    def testLookaheadWithCanonicalLR(self):
        """Only a lookahead assertion makes this grammar unambiguous."""
        tokenize = lexer.LexicalGrammar("async => { } ;", Identifier=r'\w+')
        grammar = Grammar({
            "script": [
                ["Expression", ";"],
            ],
            "Expression": [
                ["PrimaryExpression"],
                ["async", "Identifier", "=>", "AsyncConciseBody"],
            ],
            "AsyncConciseBody": [
                [LookaheadRule(set=frozenset(["{"]), positive=False), "Expression"],
                ["{", "}"],
            ],
            "PrimaryExpression": [
                ["{", "}"],
            ],
        })

        self.compile(tokenize, grammar)
        self.assertParse("{};")
        self.assertParse("async x => {};");
        self.assertParse("async x => async y => {};");

    def testMultiGoal(self):
        tokenize = lexer.LexicalGrammar("WHILE DEF FN { } ( ) -> ;", ID=r'\w+')
        grammar = Grammar({
            "stmt": [
                ["expr", ";"],
                ["{", "stmts", "}"],
                ["WHILE", "(", "expr", ")", "stmt"],
                ["DEF", "ID", "(", "ID", ")", "{", Optional("stmts"), "}"],
            ],
            "stmts": [
                ["stmt"],
                ["stmts", "stmt"],
            ],
            "expr": [
                ["FN", "ID", "->", "expr"],
                ["call_expr"],
            ],
            "call_expr": [
                ["ID"],
                ["call_expr", "(", "expr", ")"],
                ["(", "expr", ")"],
            ],
        }, goal_nts=["stmts", "expr"])
        self.compile_multi(tokenize, grammar)
        self.assertParse("WHILE ( x ) { decx ( x ) ; }", goal="stmts")
        self.assertNoParse("WHILE ( x ) { decx ( x ) ; }", goal="expr",
                           message="expected one of ['(', 'FN', 'ID'], got 'WHILE'")
        self.assertParse("f(x);", goal="stmts")
        self.assertNoParse("f(x);", goal="expr",
                           message="expected 'end of input', got ';'")
        self.assertParse("(FN x -> f ( x ))(x)", goal="expr")
        self.assertNoParse("(FN x -> f ( x ))(x)", goal="stmts",
                           message="expected ';', got None")

    def testStaggeredItems(self):
        """Two items in a state can have different amounts of leading context."""
        # In this example grammar, after "A" "B", we're in a state that
        # contains these two items (ignoring lookahead):
        #       goal ::= "A" "B" · y
        #       x ::= "B" · stars "X"
        #
        # Likewise, after `"A" "B" stars`, we have:
        #       x ::= "B" stars · "X"
        #       y ::= stars · "Y"
        #       stars ::= stars · "*"
        tokenize = lexer.LexicalGrammar("A B * X Y")
        grammar = Grammar({
            "goal": [
                ["A", "x"],
                ["A", "B", "y"],
            ],
            "x": [
                ["B", "stars", "X"],
            ],
            "y": [
                ["stars", "Y"],
            ],
            "stars": [
                ["*"],
                ["stars", "*"],
            ],
        })
        self.compile(tokenize, grammar)
        self.assertParse("A B * * * X")
        self.assertParse("A B * * * Y")

    def testCheckCycleFree(self):
        tokenize = lexer.LexicalGrammar("!")
        grammar = Grammar({
            "problem": [
                ["one", "two"],
            ],
            "one": [
                ["!"],
            ],
            "two": [
                [Optional("problem")],
            ],
        })
        self.compile(tokenize, grammar)
        self.assertParse("! ! ! ! !")

    def testReduceActions(self):
        tokenize = lexer.LexicalGrammar("+ - * / ( )", NUM=r'[0-9]\w*', VAR=r'[A-Za-z]\w*')
        grammar = Grammar({
            "expr": [
                ["term"],
                prod("expr", ["expr", "+", "term"], "add"),
                prod("expr", ["expr", "-", "term"], "sub"),
            ],
            "term": [
                ["unary"],
                prod("term", ["term", "*", "unary"], "mul"),
                prod("term", ["term", "/", "unary"], "div"),
            ],
            "unary": [
                ["prim"],
                prod("unary", ["-", "prim"], "neg"),
            ],
            "prim": [
                prod("prim", ["(", "expr", ")"], "parens"),
                prod("prim", ["NUM"], "num"),
                prod("prim", ["VAR"], "var"),
            ],
        }, goal_nts=['expr'])

        self.compile(tokenize, grammar)
        self.assertParse("1 / (1 + 1 / (1 + 1 / (1 + 1)))")
        # Not working yet!
        ## self.assertEqual(self.parse(tokenize("X")),
        ##                  ('var', 'X'))
        ## self.assertEqual(self.parse(tokenize("3 + 4")),
        ##                  ('add', ('num', '3'), '+', ('num', '4')))


if __name__ == '__main__':
    unittest.main()
