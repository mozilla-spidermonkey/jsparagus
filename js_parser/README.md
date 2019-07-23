## pgen/js_parser: Work towards generating a parser for JavaScript

In this directory:

*   **emug.pgen** A grammar for "emu-grammar", the ECMArkup grammar language.

*   **es.emug** - The actual grammar for ECMAScript, in emu-grammar
    format, extracted automatically from the spec.

*   **extract-es-grammar.py** - The script that creates *es.emug*.

*   **es-simplified.emug** - A hacked version of *es.emug* that
    jsparagus can actually handle, sort of.

*   **generate_js_parser_tables.py** - A script to generate a JS parser
    based on *es-simplified.emug*.  Read on for instructions.


## How to run it

To generate a parser, follow these steps:

```console
$ cd ..
$ python3 -m venv venv
$ . venv/bin/activate
$ pip install --upgrade pip
$ pip install -r requirements.txt
$ python -m js_parser.generate_js_parser_tables --progress -o js_parser/parser_tables.py
```

**Note:** This last step currently takes about 3 minutes to run on my
laptop.  pgen is slow.

Once you're done, to see your parser run, try this:

```console
$ python -m js_parser.try_it
```

### Or the Rust version

Run all the steps above, except substitute this command for the one that ends in `.py`:

```console
$ python -m js_parser.generate_js_parser_tables --progress --target=rust -o client/src/parser_generated.rs
```

Then, to see your parser run, try this:

```console
$ cd client
$ cargo run --release
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
