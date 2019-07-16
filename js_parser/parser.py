#!/usr/bin/env python

"""parser.py - A JavaScript parser, currently with many bugs.

See README.md for instructions.
"""

from jsparagus.runtime import Parser, ERROR, ACCEPT
from . import parser_tables
from .lexer import JSLexer


Script_entry_state = 0  # ew, magic number, get pgen to emit this


class JSParser(Parser):
    def __init__(self):
        Parser.__init__(
            self,
            parser_tables.actions,
            parser_tables.ctns,
            parser_tables.reductions,
            lambda line: JSLexer(line, self),
            Script_entry_state,
            parser_tables.DefaultBuilder()
        )


def parse_Script(text):
    return JSParser().feed(text)
