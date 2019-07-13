# Parser generator playground

This toy parser generator spits out Python 3 code.

The input to the parser generator looks like this:

```python
grammar = {
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
        ['NUMBER'],
        ['NAME'],
        ['(', 'expr', ')'],
    ],
}
```

representing this grammar:

```
expr ::= term
       | expr "+" term
       | expr "-" term

term ::= unary
       | term "*" unary
       | term "/" unary

unary ::= prim
        | "-" unary

prim ::= NUMBER
       | NAME
       | "(" expr ")"
```

It generates a table-driven shift-reduce parser:

```python
import pgen_runtime
from pgen_runtime import Apply

actions = [
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {None: -9, '*': -9, '/': -9, '+': -9, '-': -9, ')': -9},
    {None: -10, '*': -10, '/': -10, '+': -10, '-': -10, ')': -10},
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {'+': 11, '-': 12, None: -9223372036854775807},
    {'*': 13, '/': 14, None: -1, '+': -1, '-': -1, ')': -1},
    {None: -4, '*': -4, '/': -4, '+': -4, '-': -4, ')': -4},
    {None: -7, '*': -7, '/': -7, '+': -7, '-': -7, ')': -7},
    {None: -8, '*': -8, '/': -8, '+': -8, '-': -8, ')': -8},
    {')': 15, '+': 11, '-': 12},
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {'-': 1, 'NUMBER': 2, 'NAME': 3, '(': 4},
    {None: -11, '*': -11, '/': -11, '+': -11, '-': -11, ')': -11},
    {'*': 13, '/': 14, None: -2, '+': -2, '-': -2, ')': -2},
    {'*': 13, '/': 14, None: -3, '+': -3, '-': -3, ')': -3},
    {None: -5, '*': -5, '/': -5, '+': -5, '-': -5, ')': -5},
    {None: -6, '*': -6, '/': -6, '+': -6, '-': -6, ')': -6},
]

ctns = [
    {'expr': 5, 'term': 6, 'unary': 7, 'prim': 8},
    {'unary': 9, 'prim': 8},
    {},
    {},
    {'expr': 10, 'term': 6, 'unary': 7, 'prim': 8},
    {},
    {},
    {},
    {},
    {},
    {},
    {'term': 16, 'unary': 7, 'prim': 8},
    {'term': 17, 'unary': 7, 'prim': 8},
    {'unary': 18, 'prim': 8},
    {'unary': 19, 'prim': 8},
    {},
    {},
    {},
    {},
    {},
]

reductions = [
    ('expr', 1, lambda builder, x0: x0),
    ('expr', 3, lambda builder, x0, x1, x2: builder.expr_P1(x0, x1, x2)),
    ('expr', 3, lambda builder, x0, x1, x2: builder.expr_P2(x0, x1, x2)),
    ('term', 1, lambda builder, x0: x0),
    ('term', 3, lambda builder, x0, x1, x2: builder.term_P1(x0, x1, x2)),
    ('term', 3, lambda builder, x0, x1, x2: builder.term_P2(x0, x1, x2)),
    ('unary', 1, lambda builder, x0: x0),
    ('unary', 2, lambda builder, x0, x1: builder.unary_P1(x0, x1)),
    ('prim', 1, lambda builder, x0: x0),
    ('prim', 1, lambda builder, x0: x0),
    ('prim', 3, lambda builder, x0, x1, x2: builder.prim_P2(x0, x1, x2)),
]

class DefaultBuilder:
    def expr_P1(self, x0, x1, x2): return ('expr 1', x0, x1, x2)
    def expr_P2(self, x0, x1, x2): return ('expr 2', x0, x1, x2)
    def term_P1(self, x0, x1, x2): return ('term 1', x0, x1, x2)
    def term_P2(self, x0, x1, x2): return ('term 2', x0, x1, x2)
    def unary_P1(self, x0, x1): return ('unary 1', x0, x1)
    def prim_P2(self, x0, x1, x2): return ('prim 2', x0, x1, x2)

parse_expr = pgen_runtime.make_parse_fn(actions, ctns, reductions, 0, DefaultBuilder)
```

And the result of parsing the input `2 * ( x + y )` looks like this:

```python
('term 1',
    '2',
    '*',
    ('prim 2',
        '(',
        ('expr 1', 'x', '+', 'y'),
        ')'
    )
)
```

The parser spits out tuples by default. It's easy to customize the
method names of the DefaultBuilder, and it's easy to replace it with
your own builder class that produces the AST you actually want.


## Limitations

It's *all* limitations, but I'll try to list the ones that are relevant
to parsing JS.

*   Lookahead assertions are limited to one token. (The JS grammar
    contains an occasional
    ``[lookahead != `let [`]``
    and even
    ``[lookahead != `async [no LineTerminator here] function`]``.)

*   No support for any kind of dangling `else` workaround.

    Since `else` is a nonterminal, the cleanest and easiest workaround
    would be an explicit way to say in the grammar, "shifting with this
    next token is preferred over reducing with other rules".

*   There is nothing like the level of weirdness that would be needed to
    implement automatic semicolon insertion and restricted productions
    ("`[no LineTerminator here]`").

*   The input grammar can't contain actions (code to execute while parsing,
    useful for building a nice AST;
    contextually selecting the tokenizer goal symbol;
    and emitting good error messages)

*   Errors are poor:
    `(` produces "expected one of {'(', 'VAR', 'NUM', '-'}, got None".
    `)` produces "expected one of {'(', 'VAR', 'NUM', '-'}, got None".
    `a b` produces "expected one of {'-', '/', '+', '*', ')', None}, got 'VAR'".

*   Rust support is extremely rudimentary.

*   No support for feedback from syntactic context to lexical analysis
    (selecting the lexical goal symbol).

    This is needed for resolving the conflict in the lexical grammar
    between RegExps and division operators, at least. It might also be
    used for conditional keywords and for parsing TemplateTail.

*   No table compaction or table optimization. I think there's plenty of
    low-hanging fruit there.
