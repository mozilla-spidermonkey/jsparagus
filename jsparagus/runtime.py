"""Runtime support for jsparagus-generated parsers."""

# Nt is unused here, but we re-export it.
from .grammar import Nt
from .lexer import UnexpectedEndError
import collections

__all__ = ['ACCEPT', 'ERROR', 'Nt', 'Parser', 'ErrorToken']

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
    if isinstance(state, StateTermValue):
        state = state.state
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
        self.closed = False

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
        assert not self.closed

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
        assert not self.closed
        self.closed = True

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
            # lexer.take() here. Instead, call the user extension point
            # self.on_recover() to check whether it's ok to proceed. If it
            # doesn't throw, consider the ErrorToken consumed and move on.
            state = self.stack[-1]
            error_code = self.error_codes[state]
            assert error_code is not None, \
                "state that accepts an ErrorToken must have an error_code"
            self.on_recover(error_code, lexer, t)
            self.stack[-1] = action
        else:
            # 3. On error, don't attempt error handling again. Throw.
            assert action == ERROR
            self.stack[:] = saved_stack
            if t is None:
                lexer.throw_unexpected_end()
            else:
                throw_syntax_error(self.actions, self.stack[-1], t, lexer)

    def on_recover(self, error_code, lexer, t):
        """Called when the grammar says to recover from a parse error.

        Subclasses can override this to add custom code when an ErrorSymbol in
        a production is matched. This base-class implementation does nothing,
        allowing the parser to recover from the error silently.
        """
        pass

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

StateTermValue = collections.namedtuple("StateTermValue", "state term value")
class ShiftError(Exception): pass
class ShiftAccept(Exception): pass

class ParserV2:
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

    def __init__(self, actions, error_codes, entry_state, methods):
        self.actions = actions
        self.stack = [StateTermValue(entry_state, None, None)]
        self.flags = collections.defaultdict(lambda: [])
        self.error_codes = error_codes
        self.methods = methods
        self.closed = False

    def simulator_clone(self):
        """Make a copy of this parser for simulation.

        The copy has a version of the self.reductions table that never actually
        does anything.

        This is absurdly expensive and is for very odd and special use cases.
        """
        p = Parser(self.actions, self.stack[0], self.methods)
        p.stack = self.stack[:]
        return p

    def _dbg_where(self, t=""):
        print("stack: {}; {}\nexpect one of: {}".format(
            ", ".join(str(s.state) for s in self.stack), t,
            repr(self.actions[self.stack[-1].state])
        ))

    def _shift(self, t, lexer, consume_lexer = True):
        state = self.stack[-1].state
        self._dbg_where("shift: {}".format(repr(t)))
        assert isinstance(self.actions[state], dict)
        state = self.actions[state].get(t, ERROR)
        if state == ERROR:
            # self._dbg_where("(error)")
            raise ShiftError()
        if state == ACCEPT:
            # self._dbg_where("(accept)")
            raise ShiftAccept()
        if consume_lexer:
            self.stack.append(StateTermValue(state, t, lexer.take()))
        else:
            self.stack.append(StateTermValue(state, t, None))
        action = self.actions[state]
        while not isinstance(action, dict):  # Action
            # self._dbg_where("(action {})".format(state))
            action(self, lexer)
            state = self.stack[-1].state
            action = self.actions[state]
        # self._dbg_where("(shifted {})".format(t))

    def shift_list(self, stv_list, lexer):
        for stv in stv_list:
            self._shift(stv.term, lexer, False)

    def write_terminal(self, lexer, t):
        assert not self.closed
        while True:
            try:
                self._shift(t, lexer)
                break
            except ShiftError:
                self._try_error_handling(lexer, t)

    def close(self, lexer):
        assert not self.closed
        self.closed = True
        while True:
            try:
                self._shift(None, lexer, False)
                break
            except ShiftAccept:
                assert len(self.stack) == 1
                return self.stack[0].value
            except ShiftError:
                self._try_error_handling(lexer, None)

    def _try_error_handling(self, lexer, t):
        # Error recovery version of the code in write_terminal. Three differences
        # between this and write_terminal are commented below.
        assert t is not ErrorToken

        # 1.  Even if we manage to reduce successfully, undo it all on error,
        #     before throwing, to generate the best error message.
        saved_stack = self.stack[:]

        try:
            self._shift(ErrorToken, lexer)
        except ShiftError:
            # 3. On error, don't attempt error handling again. Throw.
            self.stack[:] = saved_stack
            if t is None:
                lexer.throw_unexpected_end()
            else:
                throw_syntax_error(self.actions, self.stack[-1], t, lexer)
            return

        # 2. Don't actually push an ErrorToken onto the stack or call
        # lexer.take() here. Instead, call the user extension point
        # self.on_recover() to check whether it's ok to proceed. If it
        # doesn't throw, consider the ErrorToken consumed and move on.
        state = self.stack[-1].state
        error_code = self.error_codes[state]
        assert error_code is not None, \
            "state that accepts an ErrorToken must have an error_code"
        self.on_recover(error_code, lexer, t)
        self.stack[-1] = StateTermValue(action, ErrorToken, None)

    def on_recover(self, error_code, lexer, t):
        """Called when the grammar says to recover from a parse error.

        Subclasses can override this to add custom code when an ErrorSymbol in
        a production is matched. This base-class implementation does nothing,
        allowing the parser to recover from the error silently.
        """
        pass

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
