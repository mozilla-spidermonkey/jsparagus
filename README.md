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
        ['-', 'prim'],
    ],
    'prim': [
        ['NUM'],
        ['VAR'],
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
        | "-" prim

prim ::= NUM
       | VAR
       | "(" expr ")"
```

It generates a recursive-descent parser:

```python
def parse_expr(src):
    token = src.peek()
    if token in ('-', 'NUM', '(', 'VAR'):
        result = ('expr', 0, [
            parse_term(src),
        ])
    else:
        raise ValueError('expected expr, got {!r}'.format(token))
    while True:
        token = src.peek()
        if token in ('+',):
            result = ('expr', 1, [
                result,
                src.take('+'),
                parse_term(src),
            ])
        elif token in ('-',):
            result = ('expr', 2, [
                result,
                src.take('-'),
                parse_term(src),
            ])
        else:
            break
    return result

...

def parse_unary(src):
    token = src.peek()
    if token in ('NUM', '(', 'VAR'):
        return ('unary', 0, [
            parse_prim(src),
        ])
    elif token in ('-',):
        return ('unary', 1, [
            src.take('-'),
            parse_prim(src),
        ])
    else:
        raise ValueError('expected unary, got {!r}'.format(token))

def parse_prim(src):
    token = src.peek()
    if token in ('NUM',):
        return ('prim', 0, [
            src.take('NUM'),
        ])
    elif token in ('VAR',):
        return ('prim', 1, [
            src.take('VAR'),
        ])
    elif token in ('(',):
        return ('prim', 2, [
            src.take('('),
            parse_expr(src),
            src.take(')'),
        ])
    else:
        raise ValueError('expected prim, got {!r}'.format(token))

def parse(src):
    ast = parse_expr(src)
    src.take(None)
    return ast

```

And the result of parsing the input `2 * ( x + y )` looks like this:

```python
('expr', 0, [
    ('term', 1, [
        ('term', 0, [('unary', 0, [('prim', 0, ['2'])])]),
        '*',
        ('unary', 0, [
            ('prim', 2, [
                '(',
                ('expr', 1, [
                    ('expr', 0, [('term', 0, [('unary', 0, [('prim', 1, ['x'])])])]),
                    '+',
                    ('term', 0, [('unary', 0, [('prim', 1, ['y'])])])
                ]),
                ')'
            ])
        ])
    ])
])
```

## Limitations

It's *all* limitations, but I'll try to list the ones that are relevant to parsing JS.

*   Grammar can't be indirectly left-recursive. This rules out some silly grammars like

    ```
    a ::= b ";"
    b ::= START
    b ::= a LT
    b ::= a RT
    ```

    in which *a* matches such strings as `START; LT; RT; RT;`.

    Does it matter?
    I can imagine getting a grammar like this by refactoring a directly left-recursive grammar,
    in this case:

    ```
    a ::= START ";"
    a ::= a LT ";"
    a ::= a RT ";"

*   No nonterminal can match the empty string.
    (JS nonterminal that can be empty include
    *Script*, *Module*, *FormalParameters*, and *FunctionStatementList*;
    and some lexical rules.)

*   Non-left-recursive rules for the same nonterminal must have disjoint start sets.
    You can work around this, somewhat, by doing manual left-factoring.

    But that won't always work.
    In combination with the previous limitation,
    I think this makes it impossible to represent a language that contains
    both `x` and `x y`.

*   Similarly, left-recursive rules for the same nonterminal must accept disjoint sets of tokens
    immediately following the left-recursion.

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


## What I learned, what I wonder (stab 2)

I learned it's easy for code to race ahead of understanding.
I learned that a little feature can mean a lot of complexity.

I learned that it's probably hard to support indirect left-recursion using this approach.
We're able to twist left-recursion into a `while` loop because what we're doing is local to a single nonterminal's productions,
and they're all parsed by a single function.
Making this work across function boundaries would be annoying,
even ignoring the possibility that a nonterminal can be involved in multiple left-call cycles.

I wonder if the JS spec uses any indirect left-recursion.

I wonder if there's a nice formalization of a "grammar with actions" that abstracts away "implementation details",
so that we could prove two grammars equivalent,
not just in that they describe the same language,
but equivalent in output.
This could help me explore "grammar rewrites",
which could lead to usable optimizations.

I noticed that the ES spec contains this:

> ### 13.6 The if Statement
> #### Syntax
> ```
> IfStatement[Yield, Await, Return]:
>     if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] else Statement[?Yield, ?Await, ?Return]
>     if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
> ```
>
> Each `else` for which the choice of associated `if` is ambiguous shall
> be associated with the nearest possible `if` that would otherwise have
> no corresponding `else`.

I wonder if this prose is effectively the same as adding a negative lookahead assertion
"[lookahead &ne; `else`]" at the end of the shorter production.

I wonder if follow sets can be usefully considered as context-dependent.
What do I mean by this?
For example, `function` is certainly in the follow set of *Statement* in JS,
but there are plenty of contexts, like the rule `do Statement while ( Expression ) ;`,
where the nested *Statement* is never followed by `function`.
But does it matter?
I think it only matters if you're interested in better error messages.
Follow sets only matter to detect ambiguity in a grammar,
and *Statement* is ambiguous if it's ambiguous in *any* context.


## What I learned, what I wonder (stab 1)

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

It's surprisingly tricky to check a grammar and rule out the possibility of ambiguity,
and it's easiest to do so by imposing draconian restrictions.

I learned that start sets are important even in minimal parser generators.
This is interesting because they'll be a bit more interesting to compute
once we start considering empty productions.
I wonder if it turns out to still be pretty easy.
Does the start set of a possibly-empty production include its follow set?
(According to the dragon book, you add epsilon to the start set in this case.)


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
