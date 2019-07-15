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
            lambda line: JSLexer(line, self),
            Script_entry_state,
            parser_tables.DefaultBuilder()
        )

    def can_accept(self, t):
        """Return true if the terminal `t` is OK next, false if it's an error.

        t can be None, querying if we can accept end-of-input.
        """
        state = self.simulate(t)
        action = self.actions[state].get(t, ERROR)
        if action >= 0:  # shift
            assert t is not None
            return True
        elif action == ACCEPT:
            assert t is None
            return True
        else:
            assert action == ERROR
            return False

    def can_accept_EmptyStatement(self):
        # Note: This relies on the tables not being compressed in a way that
        # loses this bit of information. Of course there are many ways to
        # compress and retain this, but the trick is making a clean interface
        # between the parser, the lexer, and the parser generator.
        state = self.simulate(";")
        return "EmptyStatement" in self.ctns[state]

    def simulate(self, t):
        """Simulate receiving the terminal `t` without modifying the parser state.

        Walk the current stack to simulate the reduce actions that would occur
        if the next token from the lexer was `t`. Return the state reached when
        we're done reducing.
        """
        stack = self.stack
        sp = len(stack) - 1
        state = stack[sp]
        while True:
            action = self.actions[state].get(t, ERROR)
            if ACCEPT < action < 0:  # reduce
                tag_name, n, _reducer = self.reductions[-action - 1]
                sp -= 2 * n
                state = stack[sp]
                sp += 2
                state = self.ctns[state][tag_name]
            else:
                return state


def parse_Script(text):
    return JSReplParser().feed(text)
