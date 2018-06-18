#!/usr/bin/env python

"""es.py - Repl-like toy to explore parsing of lines of JS.

See README.md for instructions.
"""

import sys; sys.path.append("..")

from es_parser import parse

def main():
    import lexer
    tokenize = lexer.LexicalGrammar(
        """
            { ( ) [ ]
            . ... ; , < > <= >= == != === !== + - * % ** ++ --
            << >> >>> & | ^ ! ~ && || ? :
            = += -= *= %= **= <<= >>= >>>= &= |= ^=
            =>
            / /= }
            null true false await break case catch class const continue
            debugger default delete do else export extends finally for function
            if import in instanceof new return super switch this throw try
            typeof var void while with yield enum
        """,
        NumericLiteral=r'0|[1-9][0-9]*',
        Identifier=r'[_A-Za-z]\w*'
    )

    while True:
        try:
            line = input('> ')
        except EOFError as _:
            break
        try:
            result = parse(tokenize(line))
        except Exception as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
        else:
            print(result)

if __name__ == '__main__':
    main()
