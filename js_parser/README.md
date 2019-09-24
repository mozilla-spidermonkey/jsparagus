## jsparagus/js_parser: Work towards generating a parser for JavaScript

In this directory:

*   **esgrammar.pgen** A grammar for the mini-language the ECMAScript
    standard uses to describe ES grammar.

*   **es.esgrammar** - The actual grammar for ECMAScript, in emu-grammar
    format, extracted automatically from the spec.

*   **extract-es-grammar.py** - The script that creates *es.esgrammar*.

*   **es-simplified.esgrammar** - A hacked version of *es.esgrammar* that
    jsparagus can actually handle.

*   **generate_js_parser_tables.py** - A script to generate a JS parser
    based on *es-simplified.esgrammar*.  Read on for instructions.


## How to run it

To generate a parser, follow these steps:

```console
$ cd ..
$ python3 -m venv venv
$ . venv/bin/activate
$ pip install --upgrade pip
$ pip install -r requirements.txt

$ make all
```

**Note:** The last step currently takes about 35 seconds to run on my
laptop.  jsparagus is slow.

Once you're done, to see your parser run, try this:

```console
$ python -m js_parser.try_it
```


### Or the Rust version

Run all the steps above, except substitute this command for the one that ends in `.py`:

```console
$ python -m js_parser.generate_js_parser_tables js_parser/parser_generated.jsparagus_dump -o rust/parser/src/parser_generated.rs
```

Then, to see your parser run, try this:

```console
$ cd rust/parser
$ cargo run --release
```


### How simplified is "es-simplified"?

Here are the differences between *es.esgrammar*, the actual ES grammar,
and *es-simplified.esgrammar*, the simplified version that jsparagus can
actually handle:

*   The four productions with [~Yield] and [~Await] conditions are dropped.
    This means that `yield` and `await` do not match *IdentifierReference*
    or *LabelIdentifier*. I think it's better to do that in the lexer.

*   Truncated lookahead.

    `ValueError: unsupported: lookahead > 1 token, [['{'], ['function'], ['async', ('no-LineTerminator-here',), 'function'], ['class'], ['let', '[']]`

*   Delete `[no LineTerminator here]` (restricted productions) since
    there's no way to implement it yet.

*   Delete a rule that uses `but not` since it's not implemented.

        Identifier :
          IdentifierName but not ReservedWord

    Making sense of this rule in the context of an LR parser is an
    interesting task; see issue #28.
