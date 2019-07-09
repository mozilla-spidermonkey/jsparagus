## pgen/es-grammar: Work towards generating a parser for ECMAScript

In this directory:

*   **emug.pgen** A grammar for "emu-grammar", the ECMArkup grammar language.

*   **es.emug** - The actual grammar for ECMAScript, in emu-grammar format,
    extracted automatically from the spec.

*   **extract-es-grammar.py** - The script that creates *es.emug*.

*   **es-simplified.emug** - A hacked version of *es.emug* that pgen can actually handle
    (maybe?).

*   **ponder-es-grammar.py** - A script to try generating a parser for *es-simplified.emug*.
    Read on for instructions.


## How to run it

To generate a parser, follow these steps:

```console
$ python3 -m venv venv
$ . venv/bin/activate
$ pip install --upgrade pip
$ pip install -r requirements.txt
$ ./ponder-es-grammar.py > es_parser.py
```

**Note:** This last step currently takes about 45 minutes to run on my
laptop.  pgen is slow.

Once you're done, to see your parser run, try this:

```console
$ ./es.py
```


### How simplified is "es-simplified"?

Here are the differences between *es.emug*, the actual ES grammar, and
*es-simplified.emug*, the simplified version that pgen can actually
handle:


*   Syntactic layer only.

    The simplified grammar does not contain the lexical grammar or any
    of the other various grammars in the spec (such as the RegExp grammar).

*   Truncated lookahead.

    `ValueError: unsupported: lookahead > 1 token, [['{'], ['function'], ['async', ('no-LineTerminator-here',), 'function'], ['class'], ['let', '[']]`

*   Delete `[no LineTerminator here]` (restricted productions) since
    there's no way to implement it yet.

*   Delete a rule that uses `but not` since it's not implemented.

        Identifier :
          IdentifierName but not ReservedWord

    (I question whether this is really a syntactic production. It looks
    like the kind of thing the lexical grammar ought to resolve for us.)

*   Eliminate dangling `else` problem by adding an `endif` token.

    Otherwise we'd get an error message correctly noting that the
    grammar is ambiguous as written.

    ```
    ValueError: shift-reduce conflict when looking at "if" "(" Expression_1 ")" "if" "(" Expression_1 ")" Statement_1 followed by "else"
    can't decide whether to shift into:
        IfStatement_1 ::= "if" "(" Expression_1 ")" Statement_1 "else" · Statement_1 >> {...}
    or reduce using:
        IfStatement_1 ::= "if" "(" Expression_1 ")" Statement_1

    These productions show how "else" can appear after IfStatement_1 (if we reduce):
        IfStatement_1 ::= "if" "(" Expression_1 ")" Statement_1 "else" Statement_1
        Statement_1 ::= IfStatement_1
    ```
