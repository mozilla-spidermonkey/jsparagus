"""Runtime support for jsparagus-generated parsers."""

# Nt is unused here, but we re-export it.
from .grammar import Nt
from .lexer import UnexpectedEndError

__all__ = ['ACCEPT', 'ERROR', 'Nt', 'Parser', 'ErrorToken', 'make_parse_fn']

ACCEPT = -0x7fffffffffffffff
ERROR = ACCEPT - 1


class ErrorTokenClass:
    """Special token that can be consumed to handle a syntax error."""

    def __new__(cls):
        global ErrorToken
        if ErrorToken is None:
            ErrorToken = object.__new__(ErrorTokenClass)
        return ErrorToken

    def __str__(self):
        return 'ErrorToken'

    def __repr__(self):
        # Note: If you change this, you're likely to break Python output, since
        # emit.py uses repr() in emitting parser tables.
        return 'ErrorToken'


ErrorToken = None
ErrorToken = ErrorTokenClass()


def throw_syntax_error(actions, state, t, tokens):
    assert t is not None
    expected = set(actions[state].keys())

    # Tidy up the `expected` set a bit.
    if None in expected:
        expected.remove(None)
        expected.add("end of input")
    if ErrorToken in expected:
        # This is possible because we restore the stack in _try_error_handling
        # after reducing and then failing to find a recovery rule after all.
        # But don't tell people in error messages that an error is one of the
        # things we expect. It makes no sense.
        expected.remove(ErrorToken)

    if len(expected) < 2:
        tokens.throw("expected {!r}, got {!r}".format(list(expected)[0], t))
    else:
        tokens.throw("expected one of {!r}, got {!r}"
                     .format(sorted(expected), t))


def parse(actions, ctns, reductions, error_codes, entry_state, lexer_cls, text, builder):
    """ Table-driven LR parser. """
    parser = Parser(actions,
                    ctns,
                    reductions,
                    error_codes,
                    entry_state,
                    builder)
    lexer = lexer_cls(parser)
    lexer.write(text)
    return lexer.close()


class Parser:
    """Parser using jsparagus-generated tables.

    The usual design is, a parser object consumes a token iterator.
    This Parser is not like that. Instead, the lexer feeds tokens to it
    by calling `parser.write_terminal(lexer, token)` repeatedly, then
    `parser.close(lexer)`.

    The parser uses these methods of the lexer object:

    *   lexer.take() - Return data associated with a token, like the
        numeric value of an int literal token.

    *   lexer.throw(message) - Throw a syntax error. (This is on the lexer
        because the lexer has the current position.)

    *   lexer.throw_unexpected_end() - Throw a syntax error after we
        successfully parsed the whole file except more tokens were expected at
        the end.

    """

    def __init__(self, actions, ctns, reductions, error_codes, entry_state, builder):
        self.actions = actions
        self.ctns = ctns
        self.reductions = reductions
        self.error_codes = error_codes
        self.stack = [entry_state]
        self.builder = builder

    def simulator_clone(self):
        """Make a copy of this parser for simulation.

        The copy has a version of the self.reductions table that never actually
        does anything.

        This is absurdly expensive and is for very odd and special use cases.
        """
        p = Parser(self.actions, self.ctns, self.reductions, self.error_codes,
                   self.stack[0], self.builder)
        p.stack = self.stack[:]
        # This doesn't need to be so expensive. We could proxy it instead of copying.
        p.reductions = [
            (tag_name, n, lambda *args: ())
            for tag_name, n, _reducer in self.reductions
        ]
        return p

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

    def write_terminal(self, lexer, t):
        # The loop is here for error-handling; the normal path through this
        # code reaches the `break` statement.
        while True:
            action = self._reduce(t)
            if action >= 0:  # shift
                self.stack.append(lexer.take())
                self.stack.append(action)
                break
            else:
                assert action == ERROR
                self._try_error_handling(lexer, t)

    def close(self, lexer):
        # The loop is here for error-handling only.
        while True:
            action = self._reduce(None)
            if action == ACCEPT:
                assert len(self.stack) == 3
                return self.stack[1]
            else:
                assert action == ERROR
                self._try_error_handling(lexer, None)

    def _try_error_handling(self, lexer, t):
        # Error recovery version of the code in write_terminal. Three differences
        # between this and write_terminal are commented below.
        assert t is not ErrorToken

        # 1.  Even if we manage to reduce successfully, undo it all on error,
        #     before throwing, to generate the best error message.
        saved_stack = self.stack[:]

        action = self._reduce(ErrorToken)
        if action >= 0:  # shift
            # 2. Don't actually push an ErrorToken onto the stack or call
            # lexer.take() here. Instead, call builder.handle_error() to check
            # whether it's ok to proceed. If it doesn't throw, treat the
            # ErrorToken as having been consumed and move on.
            state = self.stack[-1]
            error_code = self.error_codes[state]
            assert error_code is not None, \
                "state that accepts an ErrorToken must have an error_code"
            self.builder.handle_error(error_code)
            self.stack[-1] = action
        else:
            # 3. On error, don't attempt error handling again. Throw.
            assert action == ERROR
            self.stack[:] = saved_stack
            if t is None:
                lexer.throw_unexpected_end()
            else:
                throw_syntax_error(self.actions, self.stack[-1], t, lexer)

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
        # The easy case: no error, parsing just succeeds.
        if self.can_accept_terminal(None):
            return True

        # The hard case: maybe error-handling would succeed?
        # The easiest thing is simply to run the method.
        class BogusLexer:
            def throw_unexpected_end(self):
                raise UnexpectedEndError("")

        sim = self.simulator_clone()
        try:
            sim.close(BogusLexer())
        except SyntaxError:
            return False
        return True

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


def make_parse_fn(actions, ctns, reductions, error_codes, entry_state, builder_cls):
    def parse_fn(lexer_cls, text, builder=None):
        if builder is None:
            builder = builder_cls()
        return parse(actions, ctns, reductions, error_codes, entry_state, lexer_cls, text, builder)
    return parse_fn
