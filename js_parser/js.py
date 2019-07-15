#!/usr/bin/env python

"""js.py - Repl-like toy to explore parsing of lines of JS.

See README.md for instructions.
"""

import sys; sys.path.insert(0, "..")

from espg.pgen_runtime import ReplParser, throw_syntax_error, ERROR, ACCEPT
from js_parser import parser_tables
from js_parser.lexer import JSLexer


class JSReplParser(ReplParser):
    def __init__(self):
        ReplParser.__init__(
            self,
            parser_tables.actions,
            parser_tables.ctns,
            parser_tables.reductions,
            lambda line: JSLexer(line, self.can_accept),
            Script_entry_state,
            parser_tables.DefaultBuilder()
        )

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


def parse(actions, ctns, reductions, entry_state, text, builder):
    """ Table-driven LR parser, customized to implement ASI. """

    stack = [entry_state]  # alternates state-ids and AST nodes

    def can_accept(t):
        """Walk the stack to see if the terminal `t` is OK next or an error.

        t can be None, querying if we can accept end-of-input.
        """

        sp = len(stack) - 1
        state = stack[sp]
        while True:
            action = actions[state].get(t, ERROR)
            if action >= 0:  # shift
                return True
            elif action > ACCEPT:  # reduce
                tag_name, n, _reducer = reductions[-action - 1]
                sp -= 2 * n
                state = stack[sp]
                sp += 2
                state = ctns[state][tag_name]
            elif action == ACCEPT:
                return True
            else:
                assert action == ERROR
                return False

    tokens = JSLexer(text, can_accept)
    t = tokens.peek()

    while True:
        state = stack[-1]
        action = actions[state].get(t, ERROR)
        # possible actions are: shift, reduce, accept, error; all encoded as integers
        if action >= 0:  # shift
            stack.append(tokens.take(t))
            stack.append(action)
            t = tokens.peek()
        elif action > ACCEPT:  # reduce
            tag_name, n, reducer = reductions[-action - 1]
            start = len(stack) - 2 * n
            node = reducer(builder, *stack[start::2])
            stack[start:] = [node, ctns[stack[start - 1]][tag_name]]
        elif action == ACCEPT:
            assert len(stack) == 3
            return stack[1]
        else:
            assert action == ERROR
            throw_syntax_error(actions, state, t, tokens)


Script_entry_state = 0 # ew, magic number, get pgen to emit this


def parse_Script(line):
    return parse(
        parser_tables.actions,
        parser_tables.ctns,
        parser_tables.reductions,
        Script_entry_state,
        line,
        parser_tables.DefaultBuilder()
    )

def main():
    while True:
        parser = JSReplParser()
        try:
            result = parser.read()
        except SyntaxError as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
            continue
        print(result)

if __name__ == '__main__':
    main()
