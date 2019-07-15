"""Runtime support for generated parsers."""

from .grammar import Apply  # to re-export
from .lexer import UnexpectedEndError

__all__ = ['ACCEPT', 'ERROR', 'Apply', 'ReplParser', 'make_parse_fn']

ACCEPT = -0x7fffffffffffffff
ERROR = ACCEPT - 1


def throw_syntax_error(actions, state, t, tokens):
    expected = set(actions[state].keys())
    if None in expected:
        expected.remove(None)
        expected.add("end of input")
    elif t is None:
        tokens.throw_unexpected_end()
    if len(expected) < 2:
        tokens.throw("expected {!r}, got {!r}".format(list(expected)[0], t))
    else:
        tokens.throw("expected one of {!r}, got {!r}"
                     .format(sorted(expected), t))


def parse(actions, ctns, reductions, entry_state, tokens, builder):
    """ Table-driven LR parser. """
    t = tokens.peek()
    stack = [entry_state]  # alternates state-ids and AST nodes
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


class ReplParser:
    """Object for parsing a single Script that may cover multiple lines.

    Usage: Call .feed(line) repeatedly until either it returns an AST or raises
    an exception other than jsparagus.lexer.UnexpectedEndError.

    Alternatively, just call .read() once, which does all of that, reading
    input lines from stdin as needed.
    """

    def __init__(self, actions, ctns, reductions, lex, entry_state, builder):
        self.actions = actions
        self.ctns = ctns
        self.reductions = reductions
        self.lex = lex
        self.stack = [entry_state]
        self.builder = builder

    def read(self, prompt="> "):
        while True:
            line = input(prompt)
            try:
                return self.feed(line + "\n")
            except UnexpectedEndError:
                prompt = "» "
                continue

    def feed(self, line):
        stack = self.stack
        tokens = self.lex(line)
        t = tokens.peek()
        while True:
            state = stack[-1]
            action = self.actions[state].get(t, ERROR)
            if action >= 0:  # shift
                stack.append(tokens.take(t))
                stack.append(action)
                t = tokens.peek()
                if t is None:
                    # cope with end of line somehow
                    pass
            elif action > ACCEPT:  # reduce
                tag_name, n, reducer = self.reductions[-action - 1]
                start = len(stack) - 2 * n
                node = reducer(self.builder, *stack[start::2])
                stack[start:] = [node, self.ctns[stack[start - 1]][tag_name]]
            elif action == ACCEPT:
                assert len(stack) == 3
                return stack[1]
            else:
                assert action == ERROR
                throw_syntax_error(self.actions, state, t, tokens)


def make_parse_fn(actions, ctns, reductions, entry_state, builder_cls):
    def parse_fn(tokens, builder=None):
        if builder is None:
            builder = builder_cls()
        return parse(actions, ctns, reductions, entry_state, tokens, builder)
    return parse_fn
