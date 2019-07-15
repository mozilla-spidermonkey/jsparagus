#!/usr/bin/env python

"""parser.py - A JavaScript parser, currently with many bugs.

See README.md for instructions.
"""

from jsparagus.pgen_runtime import (ReplParser, throw_syntax_error,
                                    ERROR, ACCEPT)
from . import parser_tables
from .lexer import JSLexer


Script_entry_state = 0 # ew, magic number, get pgen to emit this


class JSReplParser(ReplParser):
    def __init__(self):
        ReplParser.__init__(
            self,
            parser_tables.actions,
            parser_tables.ctns,
            parser_tables.reductions,
            lambda line: JSLexer(line, self.can_accept,
                                 self.can_accept_EmptyStatement),
            Script_entry_state,
            parser_tables.DefaultBuilder()
        )

    def can_accept_EmptyStatement(self):
        # Note: This relies on the tables not being compressed in a way that
        # loses this bit of information. Of course there are many ways to
        # compress and retain this, but the trick is making a clean interface
        # between the parser, the lexer, and the parser generator.
        state = self.stack[-1]
        return "EmptyStatement" in self.ctns[state]

    def can_accept(self, t):
        """Walk the stack to see if the terminal `t` is OK next or an error.

        t can be None, querying if we can accept end-of-input.
        """
        stack = self.stack
        sp = len(stack) - 1
        state = stack[sp]
        while True:
            action = self.actions[state].get(t, ERROR)
            if action >= 0:  # shift
                return True
            elif action > ACCEPT:  # reduce
                tag_name, n, _reducer = self.reductions[-action - 1]
                sp -= 2 * n
                state = stack[sp]
                sp += 2
                state = self.ctns[state][tag_name]
            elif action == ACCEPT:
                return True
            else:
                assert action == ERROR
                return False


def parse_Script(text):
    return JSReplParser().feed(text)
