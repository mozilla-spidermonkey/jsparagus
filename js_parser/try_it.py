#!/usr/bin/env python

"""js.py - Repl-like toy to explore parsing of lines of JS.

See README.md for instructions.
"""

import argparse
from .parser import JSParser
from jsparagus.lexer import SyntaxError


def interactive_input(parser, prompt="js> "):
    while True:
        line = input(prompt)
        parser.write(line + "\n")
        if parser.can_close():
            return parser.close()
        prompt = "..> "

def rpl():
    """Read-print loop."""
    while True:
        parser = JSParser()
        try:
            result = interactive_input(parser)
        except EOFError:
            print()
            break
        except SyntaxError as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
            continue
        print(result)

def main():
    parser = argparse.ArgumentParser(description="Try out the JS parser.")
    parser.add_argument('input_file', metavar='FILE', nargs='?',
                        help=".js file to parse")
    options = parser.parse_args()

    if options.input_file is not None:
        with open(options.input_file) as f:
            source = f.readlines()
        parser = JSParser()
        for line in source:
            print(line.rstrip())
            parser.write(line)
        ast = parser.close()
        print(ast)
    else:
        rpl()


if __name__ == '__main__':
    main()
