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

It generates a rather strange-looking recursive-descent parser:

```python
def parse_expr(src, stack):
    token = src.peek()
    if token in ('NUM', '(', 'VAR'):
        parse_term(src, stack)
        args = stack[-1:]
        del stack[-1:]
        stack.append(('expr', 0, args))
        parse_expr_(src, stack)
    else:
        raise ValueError('expected expr, got {!r}'.format(token))

...

def parse_prim(src, stack):
    token = src.peek()
    if token == 'NUM':
        stack.append(src.take('NUM'))
        args = stack[-1:]
        del stack[-1:]
        stack.append(('prim', 0, args))
    elif token == 'VAR':
        stack.append(src.take('VAR'))
        args = stack[-1:]
        del stack[-1:]
        stack.append(('prim', 1, args))
    elif token == '(':
        stack.append(src.take('('))
        parse_expr(src, stack)
        stack.append(src.take(')'))
        args = stack[-3:]
        del stack[-3:]
        stack.append(('prim', 2, args))
    else:
        raise ValueError('expected prim, got {!r}'.format(token))

def parse_expr_(src, stack):
    token = src.peek()
    if token == '+':
        stack.append(src.take('+'))
        parse_term(src, stack)
        args = stack[-3:]
        del stack[-3:]
        stack.append(('expr', 1, args))
        parse_expr_(src, stack)
    elif token == '-':
        stack.append(src.take('-'))
        parse_term(src, stack)
        args = stack[-3:]
        del stack[-3:]
        stack.append(('expr', 2, args))
        parse_expr_(src, stack)

...

def parse(src):
    stack = []
    parse_expr(src, stack)
    src.take(None)
    assert len(stack) == 1
    return stack[0]
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

*   No nonterminal in the input grammar can match the empty string.
    (JS nonterminal that can be empty include
    *Script*, *Module*, *FormalParameters*, and *FunctionStatementList*;
    and some lexical rules.)

    (Internally, the parser generator does transformations that can
    result in empty productions, but supporting them in the input grammar
    would complicate some things.)

*   The grammar must be LL(1), after automated left-recursion
    elimination and left-factoring.

*   No Kleene quantifiers.

*   The input grammar can't contain actions (code to execute while parsing,
    useful for building a nice AST;
    contextually selecting the tokenizer goal symbol;
    and emitting good error messages)

*   The output is a fairly literal-minded parse tree, really too literal.

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

### Stab 3

I learned how to eliminate left recursion in a grammar (Algorithm 4.1
from the book). I learned how to check that a grammar is LL(1) using
the start and follow sets, although I didn't really learn what LL(1)
means in any depth. (I'm just using it as a means to prove that the
grammar is unambiguous.)

I learned from the book how to do a table-driven "nonrecursive
predictive parser". Something to try later.

I came up with the "reduction symbol" thing. It seems to work as
expected!  This allows me to transform the grammar, but still generate
parse trees reflecting the source grammar. However, the resulting code
is inefficient. Further optimization would improve it, but the
predictive parser will fare better even without optimization.

I wonder what differences there are between LL(1) and LR(1) grammars.

I have forgotten what LALR stands for.


### Stab 2

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

(I asked bterlson and he thinks so.)

I wonder if follow sets can be usefully considered as context-dependent.
What do I mean by this?
For example, `function` is certainly in the follow set of *Statement* in JS,
but there are plenty of contexts, like the rule `do Statement while ( Expression ) ;`,
where the nested *Statement* is never followed by `function`.
But does it matter?
I think it only matters if you're interested in better error messages.
Follow sets only matter to detect ambiguity in a grammar,
and *Statement* is ambiguous if it's ambiguous in *any* context.


### Stab 1

I learned that if you simply define a grammar as a set of rules,
there are all sorts of anomalies that can come up:

*   Vacant nonterminals (that do not match any input strings);

*   Nonterminals that match only infinite strings, like `a ::= X a`.

*   Cycles ("busy loops"), like `a ::= a`.
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
