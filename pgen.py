#!/usr/bin/env python3

"""pgen.py - Generate a parser from a pgen grammar.

(This is for testing. pgen will likely go away. Ignore this for now.)
"""

import argparse
import parse_pgen
import gen
import sys

def main():
    parser = argparse.ArgumentParser(description="Generate a parser.")
    parser.add_argument('grammar', metavar='FILE', nargs=1,
                        help=".pgen file containing the grammar")
    args = parser.parse_args()

    [pgen_filename] = args.grammar
    grammar = parse_pgen.load_grammar(pgen_filename)
    gen.generate_parser(sys.stdout, grammar, next(iter(grammar)))


if __name__ == '__main__':
    main()
