#!/usr/bin/env python

""" ponder-es-grammar.py - Try to parse the ES grammar. """

import sys; sys.path.append("..")

import gen
import argparse
import parse_emug


ECMASCRIPT_GOAL_NTS = [
    'Script',
    'Module',
    #'FunctionBody[~Yield, ~Await]',
    #'GeneratorBody',
    #'AsyncFunctionBody',
    #'AsyncGeneratorBody',
    #'FormalParameters[~Yield, ~Await]',
    #'FormalParameters[~Yield, +Await]',
    #'FormalParameters[+Yield, ~Await]',
    #'FormalParameters[+Yield, +Await]',
]


def main():
    parser = argparse.ArgumentParser(description='Ponder the ECMAScript grammar.',
                                     allow_abbrev=False)
    parser.add_argument('filename', metavar='FILE', nargs='?', default='./es-simplified.emug',
                        help="emug file containing the grammar")
    parser.add_argument('-o', '--output', metavar='FILE', default='/dev/stdout',
                        help="output filename for parser tables")
    parser.add_argument('--target', choices=['python', 'rust'], default='python',
                        help="target language to use when printing the parser tables")
    parser.add_argument('-v', '--verbose', action='store_true',
                        help="print some debug output")
    parser.add_argument('--progress', action='store_true',
                        help="print a dot each time a state is analyzed (thousands of them)")
    args = parser.parse_args()

    with open(args.filename) as f:
        text = f.read()

    grammar = parse_emug.parse_emug(text, filename=args.filename)
    if args.verbose:
        grammar.dump()

    with open(args.output, 'w') as f:
        gen.generate_parser(f, grammar, ECMASCRIPT_GOAL_NTS, target=args.target,
                            verbose=args.verbose, progress=args.progress)


if __name__ == '__main__':
    main()
