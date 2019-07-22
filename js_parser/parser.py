#!/usr/bin/env python

"""parser.py - A JavaScript parser, currently with many bugs.

See README.md for instructions.
"""

from jsparagus.runtime import Parser, ERROR, ACCEPT
from jsparagus.lexer import SyntaxError
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
            self._new_lexer,
            Script_entry_state,
            parser_tables.DefaultBuilder()
        )
        self.has_written = False

    def _new_lexer(self, line):
        is_extra_line = self.has_written
        self.has_written = True
        return JSLexer(self)

    def write(self, text):
        raise TypeError("this is not a standard parser")

    def can_close(self):
        """Override the base-class parser to cope with ASI in JS."""
        # The easy case: the parser actually accepts None from here.
        if super().can_close():
            return True

        # The hard case: maybe ASI would work?
        sim = self.simulator_clone()
        bogus_lexer = JSLexer(sim)
        try:
            bogus_lexer.write(";")
            bogus_lexer.close()
        except SyntaxError as exc:
            # If one isn't enough, more semicolons can't succeed.
            return False
        return True

    def asi(self):
        """Insert a semicolon."""
        bogus_lexer = JSLexer(self)
        bogus_lexer.write("; ")

    def on_syntax_error(self, tokens, t):
        if t == '}' or tokens.saw_line_terminator():
            # Implement ASI at this point.
            if (self.can_accept_terminal(';') and
                    not self.can_accept_nonterminal('EmptyStatement', ';')):
                self.asi()
                return 'retry'
        super().on_syntax_error(tokens, t)

    def close(self):
        # Implement ASI at the end of the Script.
        if (not super().can_close() and
                self.can_accept_terminal(';') and
                not self.can_accept_nonterminal('EmptyStatement', ';')):
            self.asi()

        return super().close()

def parse_Script(text):
    lexer = JSLexer(JSParser())
    lexer.write(text)
    return lexer.close()
