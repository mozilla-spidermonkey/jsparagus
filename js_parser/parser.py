#!/usr/bin/env python

"""parser.py - A JavaScript parser, currently with many bugs.

See README.md for instructions.
"""

import jsparagus
from . import parser_tables
from .lexer import JSLexer


Script_entry_state = 0  # ew, magic number, get pgen to emit this


class JSParser(jsparagus.runtime.Parser):
    def __init__(self):
        jsparagus.runtime.Parser.__init__(
            self,
            parser_tables.actions,
            parser_tables.ctns,
            parser_tables.reductions,
            Script_entry_state,
            parser_tables.DefaultBuilder()
        )


def parse_Script(text):
    lexer = JSLexer(JSParser())
    lexer.write(text)
    return lexer.close()
