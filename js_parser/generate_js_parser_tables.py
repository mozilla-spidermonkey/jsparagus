"""generate_js_parser_tables.py - Generate tables from the ES grammar."""

import argparse
import os
import jsparagus.gen
from .parse_esgrammar import parse_esgrammar


ECMASCRIPT_GOAL_NTS = [
    'Script',
    'Module',
    # 'FunctionBody[~Yield, ~Await]',
    # 'GeneratorBody',
    # 'AsyncFunctionBody',
    # 'AsyncGeneratorBody',
    # 'FormalParameters[~Yield, ~Await]',
    # 'FormalParameters[~Yield, +Await]',
    # 'FormalParameters[+Yield, ~Await]',
    # 'FormalParameters[+Yield, +Await]',
]


def main():
    parser = argparse.ArgumentParser(
        description='Ponder the ECMAScript grammar.',
        allow_abbrev=False)
    default_filename = os.path.join(os.path.dirname(__file__),
                                    "es-simplified.esgrammar")
    parser.add_argument(
        'filename', metavar='FILE', nargs='?', default=default_filename,
        help="esgrammar input file")
    parser.add_argument(
        '-o', '--output', metavar='FILE', default='/dev/stdout',
        help="output filename for parser tables")
    parser.add_argument(
        '--target', choices=['python', 'rust'], default='python',
        help="target language to use when printing the parser tables")
    parser.add_argument(
        '-v', '--verbose', action='store_true',
        help="print some debug output")
    parser.add_argument(
        '--progress', action='store_true',
        help="print a dot each time a state is analyzed (thousands of them)")
    args = parser.parse_args()

    with open(args.filename) as f:
        text = f.read()

    grammar = parse_esgrammar(text, filename=args.filename,
                              goals=ECMASCRIPT_GOAL_NTS)
    if args.verbose:
        grammar.dump()

    with open(args.output, 'w') as f:
        jsparagus.gen.generate_parser(f, grammar,
                                      target=args.target,
                                      verbose=args.verbose,
                                      progress=args.progress)


if __name__ == '__main__':
    main()
