"""Runtime support for jsparagus-generated parsers."""

from .grammar import Apply  # to re-export
from .lexer import UnexpectedEndError

__all__ = ['ACCEPT', 'ERROR', 'Apply', 'Parser', 'make_parse_fn']

ACCEPT = -0x7fffffffffffffff
ERROR = ACCEPT - 1


def throw_syntax_error(actions, state, t, tokens):
    assert t is not None
    expected = set(actions[state].keys())
    if None in expected:
        expected.remove(None)
        expected.add("end of input")
    if len(expected) < 2:
        tokens.throw("expected {!r}, got {!r}".format(list(expected)[0], t))
    else:
        tokens.throw("expected one of {!r}, got {!r}"
                     .format(sorted(expected), t))


def parse(actions, ctns, reductions, entry_state, tokens, builder):
    """ Table-driven LR parser. """
    parser = Parser(actions,
                    ctns,
                    reductions,
                    lambda _ignored_bogus_text: tokens,
                    entry_state,
                    builder)
    parser.write("")
    return parser.close()


class Parser:
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

    def _reduce(self, t):
        stack = self.stack
        state = stack[-1]
        action = self.actions[state].get(t, ERROR)
        while ACCEPT < action < 0:  # reduce
            tag_name, n, reducer = self.reductions[-action - 1]
            start = len(stack) - 2 * n
            node = reducer(self.builder, *stack[start::2])
            stack[start:] = [node, self.ctns[stack[start - 1]][tag_name]]
            state = stack[-1]
            action = self.actions[state].get(t, ERROR)
        return action

    def write(self, chunk):
        tokens = self.lex(chunk)
        t = tokens.peek()
        while t is not None:
            self.write_terminal(tokens, t)
            t = tokens.peek()

    def write_terminal(self, tokens, t):
        action = self._reduce(t)
        if action >= 0:  # shift
            self.stack.append(tokens.take(t))
            self.stack.append(action)
        else:
            assert action == ERROR
            throw_syntax_error(self.actions, self.stack[-1], t, tokens)

    def close(self):
        action = self._reduce(None)
        if action == ACCEPT:
            assert len(self.stack) == 3
            return self.stack[1]
        else:
            assert action == ERROR
            self.lex("").throw_unexpected_end()

    def can_accept_terminal(self, t):
        """Return True if the terminal `t` is OK next.

        False if it's an error. `t` can be None, querying if we can accept
        end-of-input.
        """
        state = self.simulate(t)
        action = self.actions[state].get(t, ERROR)
        return action != ERROR

    def can_close(self):
        """Return True if self.close() would succeed."""
        return self.can_accept_terminal(None)

    def can_accept_nonterminal(self, nt, t):
        """Return True if a nonterminal `nt` starting with `t` is OK next.

        The starting terminal `t` helps in navigating the parser tables;
        without it, there are too many ways parsing could proceed from the
        current state, and the check would be slow--it would have to
        brute-force the state machine.
        """
        # Note: This relies on the tables not being compressed in a way that
        # loses this bit of information. Of course there are many ways to
        # compress and retain this; the only trick is making a clean interface
        # between the parser, the lexer, and the parser generator.
        state = self.simulate(t)
        return nt in self.ctns[state]

    def simulate(self, t):
        """Simulate receiving the terminal `t` without modifying parser state.

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


def make_parse_fn(actions, ctns, reductions, entry_state, builder_cls):
    def parse_fn(tokens, builder=None):
        if builder is None:
            builder = builder_cls()
        return parse(actions, ctns, reductions, entry_state, tokens, builder)
    return parse_fn
