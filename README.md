# Parser generator playground

This toy parser generator spits out Python 3 code.

The input to the parser generator looks like this:

```python
grammar = {
    'sexpr': [
        ['Symbol'],
        ["(", 'tail'],
    ],
    'tail': [
        [")"],
        ['sexpr', 'tail']
    ],
}
```

representing this grammar:

```
sexpr ::= SYMBOL
sexpr ::= "(" tail

tail ::= ")"
tail ::= sexpr tail
```

It generates a recursive-descent parser:

```python
def parse_sexpr(src):
    token = src.peek()
    if token in ('Symbol',):
        return ('sexpr', 0, [
            src.take('Symbol'),
        ])
    elif token in ('(',):
        return ('sexpr', 1, [
            src.take('('),
            parse_tail(src),
        ])
    else:
        raise ValueError('expected sexpr, got {!r}'.format(token))

def parse_tail(src):
    token = src.peek()
    if token in (')',):
        return ('tail', 0, [
            src.take(')'),
        ])
    elif token in ('Symbol', '('):
        return ('tail', 1, [
            parse_sexpr(src),
            parse_tail(src),
        ])
    else:
        raise ValueError('expected tail, got {!r}'.format(token))

def parse(src):
    ast = parse_sexpr(src)
    src.take(None)
    return ast
```

And the result of parsing the input `( lambda ( x ) ( * x x ) )` is this mess:

```python
('sexpr', 1, [
    '(',
    ('tail', 1, [
        ('sexpr', 0, ['lambda']),
        ('tail', 1, [
            ('sexpr', 1, [
                '(',
                ('tail', 1, [
                    ('sexpr', 0, ['x']),
                    ('tail', 0, [')'])
                ])
            ]),
            ('tail', 1, [
                ('sexpr', 1, [
                    '(',
                    ('tail', 1, [
                        ('sexpr', 0, ['*']),
                        ('tail', 1, [
                            ('sexpr', 0, ['x']),
                            ('tail', 1, [
                                ('sexpr', 0, ['x']),
                                ('tail', 0, [')'])
                            ])
                        ])
                    ])
                ]),
                ('tail', 0, [')'])
            ])
        ])
    ])
])
```

## Limitations

It's *all* limitations, but I'll try to list the ones that are relevant to parsing JS.

*   Grammar can't be left-recursive, directly or indirectly.
    This is a severe restriction; I think it rules out a lot of languages. Not sure.

*   No nonterminal can match the empty string.
    (JS nonterminal that can be empty include
    *Script*, *Module*, *FormalParameters*, and *FunctionStatementList*;
    and some lexical rules.)

*   No two rules for the same nonterminal can have overlapping start sets.
    You can work around this, somewhat, by doing manual left-factoring.

    But that won't always work.
    In combination with the previous limitation,
    I think this makes it impossible to represent a language that contains
    both `x` and `x y`.

*   No Kleene quantifiers.

*   Grammar rules can't contain actions (code to execute while parsing,
    useful for building a nice AST;
    contextually selecting the tokenizer goal symbol;
    and emitting good error messages)

*   The output is a very literal-minded parse tree, really too literal.

*   Recursive descent parsers can run out of stack.
    The parser above will throw a RecursionError if asked to parse
    a Lisp list with a thousand elements: `(1 2 3 4 ... 1000)`.

Minor items:

*   Only supports a single goal nonterminal (easy to fix).

*   No location information (ditto).

*   Errors are poor:
    `(` produces "expected tail, got None";
    `)` produces "expected sexpr, got ')'";
    `a b` produces "expected end of input".

*   Optimization opportunity:
    Some of the `token in (...)` tests this emits are redundant.
    For rules like `tail ::= sexpr tail`, we can turn that `elif token in ('Symbol', '(')` into an `else`.
    Apart from the wording of the error message, there's no difference.

*   No support for the JS parsing curiosities:

    *   feedback from syntactic context to lexical analysis
        (selecting the lexical goal symbol)

    *   boolean parameterization of nonterminals (`[Yield]` and such)
    
    *   automatic semicolon insertion
    
    *   "[lookahead not in {`let`, `[`}]"

    *   dangling `else`


## What I learned, what I wonder

I learned that if you simply define a grammar as a set of rules,
there are all sorts of anomalies that can come up:

*   Vacant nonterminals (that do not match any input strings);

*   Nonterminals that match only infinite strings;

*   "Busy loops", like `a ::= a`.
    These always introduce ambiguity.
    (You can also have cycles through multiple nonterminals:
    `a ::= b; b ::= a`.)

These in particular are easy to test for, with no false positives.
I wonder if there are other anomalies,
and if the "easiness" generalizes to all of them, and why.

I know what it means for a grammar to be *ambiguous*:
it means there's at least one input with multiple valid parses.
I understand that parser generators can check for ambiguity.
But it's easiest to do so by imposing draconian restrictions.
I learned the "dangling `else` problem" is an ambiguity in exactly this sense.
I wonder if there's a principled way to deal with it.

I know that a parse is a constructive proof that a string matches a grammar.

It's tricky to check a grammar and rule out the possibility of ambiguity,
and it's easiest to do so by imposing draconian restrictions.

I learned that start sets are important even in minimal parser generators.
This is interesting because they'll be a bit more interesting to compute
once we start considering empty productions.
I wonder if it turns out to still be pretty easy.
Does the start set of a possibly-empty production include its follow set?
(No.)


### Nice grammars

I learned that the definition of a "grammar"
as a formal description of a language (= a set of strings)
is incomplete.

Consider the Lisp syntax we're using:

```
sexpr ::= SYMBOL
sexpr ::= "(" tail

tail ::= ")"
tail ::= sexpr tail
```

Nobody wants to parse Lisp like that.
There are two problems.

One is expressive.
The `"("` and `")"` tokens should appear in the same production.
That way, the grammar says declaratively: these marks always appear in properly nesting pairs.

```
sexpr ::= SYMBOL
sexpr ::= "(" list ")"

list ::= [empty]
list ::= sexpr list
```

The other problem has to do with *what you've got* when you get an automatically generated parse.
A grammar is more than just a description of a language,
to the extent we care about the form of the parse trees we get out of the parser.

A grammar is a particular way of writing a parser,
and since we care about the parser's output,
we care about details of the grammar that would be mere "implementation details" otherwise.
