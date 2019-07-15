#!/usr/bin/env python3

""" extract-es-grammar.py - Extract the grammar from the ECMAScript spec

To run this script:

    python3 -m venv venv
    . venv/bin/activate
    pip install --upgrade pip
    pip install -r requirements.txt
    ./extract-es-grammar.py path/to/tc39/ecma262/spec.html > es.emug

"""

import html5lib
from textwrap import dedent


def extract(filename):
    with open(filename, "rb") as f:
        document = html5lib.parse(f)

    for e in document.iter("{http://www.w3.org/1999/xhtml}emu-grammar"):
        if e.attrib.get("type") == "definition":
            print(dedent(e.text))


if __name__ == '__main__':
    import sys
    if len(sys.argv) < 2:
        print("usage:  ./extract-es-grammar.py path/to/tc39/ecma262/spec.html")
    else:
        extract(sys.argv[1])
