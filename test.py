#!/usr/bin/env python3

import gen
import io, unittest

class ListTokens:
    def __init__(self, tokens):
        self.tokens = tokens

    def peek(self):
        if len(self.tokens) == 0:
            return None
        else:
            next = self.tokens[0]
            if next.isdigit():
                return "NUM"
            elif next.isalpha():
                return "VAR"
            elif next in '+-*/()':
                return next
            else:
                raise ValueError("unexpected token {!r}".format(next))

    def take(self, k):
        if k is None:
            if self.tokens:
                raise ValueError("expected end of input")
        else:
            assert self.peek() == k
            return self.tokens.pop(0)

class LetterTokens(ListTokens):
    def __init__(self, s):
        ListTokens.__init__(self, list(s))

class SplitTokens(ListTokens):
    def __init__(self, s):
        ListTokens.__init__(self, s.split())

class LispTokens(SplitTokens):
    def peek(self):
        if len(self.tokens) == 0:
            return None
        else:
            next = self.tokens[0]
            if next in '()':
                return next
            else:
                return 'SYMBOL'

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

        parsed = parse(LispTokens("( lambda ( x ) ( * x x ) )"))
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
        tokens = SplitTokens('2 * 3 + 4 * ( 5 + 7 )')
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
        grammar = {
            'goal': [
                ['+'],
                ['+', '-'],
            ],
        }

        parse = compile(grammar, goal='goal')
        self.assertEqual(parse(SplitTokens("+")), ('goal', 0, ['+']))
        self.assertEqual(parse(SplitTokens("+ -")), ('goal', 1, ['+', '-']))


if __name__ == '__main__':
    unittest.main()
