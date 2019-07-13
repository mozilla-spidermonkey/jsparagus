#!/usr/bin/env python

"""es.py - Repl-like toy to explore parsing of lines of JS.

See README.md for instructions.
"""

import sys; sys.path.append("..")

import re
from espg.lexer import SyntaxError
from espg.pgen_runtime import ERROR, ACCEPT
import es_parser

TOKEN_RE = re.compile(r'''(?x)
  (?:
      # WhiteSpace
      [\ \t\v\r\n\u00a0\ufeff]
      # SingleLineComment
    | // [^\r\n\u2028\u2029]*
      # MultiLineComment
    | /\*  (?: [^*] | \*+[^/] )*  \*+/
  )*
  (
      # IdentifierName
      (?: [$_A-Za-z]     | \\ u [0-9A-Fa-f]{4} | \\ u \{ [0-9A-Fa-f]+ \})
      (?: [$_0-9A-Za-z]  | \\ u [0-9A-Fa-f]{4} | \\ u \{ [0-9A-Fa-f]+ \})*
      # NumericLiteral
    | [0-9][0-9A-Za-z]*(?:\.[0-9A-Za-z]*)?
    | \.[0-9]+
      # Punctuator
    | [ { ( ) \[ \] ; , ~ \? :]
    | \.\.\. | \. | <<=? | <= | < | >>>=? | >>=? | >= | > | === | == | = | !== | != | !
    | \+\+ | \+=? | -- | -=? | \*\*=? | \*=? | %=? | && | &=? | \^=? | \|\| | \|=? | =>
      # The slash special case
    | /
      # The curly brace special case
    | }
      # StringLiteral
    | ' (?: [^'\\\r\n] | \\['"] | \\x[0-9A-Fa-f]{2} | \\u[0-9A-Fa-f]{4} | \\u\{[0-9A-Fa-f]+\}
         | \\\r\n? | \\[\n\u2028\u2029] )* ' # TODO finish list of escapes
    | " (?: [^"\\\r\n] | \\['"] | \\x[0-9A-Fa-f]{2} | \\u[0-9A-Fa-f]{4} | \\u\{[0-9A-Fa-f]+\}
         | \\\r\n? | \\[\n\u2028\u2029] )* " # TODO finish list of escapes
      # Template
    | ` (?: [^`\\$] | \\. )* (?: \${ | ` )
      # Any other character is an error.
    | .
    | \Z #end of string
  )
''')

RESERVED_WORDS = set('''
await break case catch class const continue debugger default delete do else
export extends finally for function if import in instanceof new return super
switch this throw try typeof var void while with yield
enum
null true false
endif
'''.split())

class ESLexer:
    """Vague approximation of an ECMAScript lexer. """
    def __init__(self, source, parser_can_accept, filename=None):
        self.src = source
        self.filename = filename
        self.last_point = 0
        self.point = 0
        self._next_kind = None
        self.parser_can_accept = parser_can_accept

    def _match(self):
        match = self._next_match = TOKEN_RE.match(self.src, self.point)
        assert match is not None, "TOKEN_RE should always match"
        token = match.group(1)
        self.point = match.start(1)

        if token == '':
            assert match.end() == len(self.src)
            # Implement ASI at the end of the program.
            if not self.parser_can_accept(None) and self.parser_can_accept(';'):
                return ';'
            return None
        c = token[0]
        if c.isdigit():
            return 'NumericLiteral'
        elif c.isalpha() or c in '$_':
            if self.parser_can_accept('IdentifierName'):
                return 'IdentifierName'
            elif token in RESERVED_WORDS:  # TODO support strict mode
                if token == 'null':
                    return 'NullLiteral'
                elif token in ('true', 'false'):
                    return 'BooleanLiteral'
                return token
            elif token in ('let', 'static', 'yield', 'async', 'of') and self.parser_can_accept(token):
                # This is not what the standard says but eh
                return token
            else:
                return 'Identifier'
        elif c == '/':
            if self.parser_can_accept('RegularExpressionLiteral'):
                DIE
            else:
                match = self._next_match = re.match(r'(/=?)', self.src, self.point)
                token = match.group(1)
            return token
        elif c.startswith('`'):
            if c.endswith('`'):
                return 'NoSubstitutionTemplate'
            else:
                return 'TemplateHead'
        elif c == '}':
            return token
        else:
            return token

    def peek(self):
        if self._next_kind is not None:
            return self._next_kind
        self.last_point = self.point
        hit = self._next_kind = self._match()
        if hit is None:
            return None
        self.last_point = self._next_match.end()
        return hit

    def take(self, k):
        match = self._next_match
        self.point = match.end()
        self._next_kind = None
        self._next_match = None
        return match.group(1)

    def throw(self, msg):
        e = SyntaxError(msg)
        e.filename = self.filename
        e.lineno, e.column = self.last_point_coords()
        raise e

    def last_point_coords(self):
        src_pre = self.src[:self.last_point]
        lineno = 1 + src_pre.count("\n")
        line_start_index = src_pre.rfind("\n") + 1
        column = self.last_point - line_start_index  # can be zero
        return lineno, column

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

    tokens = ESLexer(text, can_accept)
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
            expected = set(actions[state].keys())
            if None in expected:
                expected.remove(None)
                expected.add("end of input")
            if len(expected) < 2:
                tokens.throw("expected {!r}, got {!r}".format(list(expected)[0], t))
            else:
                tokens.throw("expected one of {!r}, got {!r}"
                             .format(sorted(expected), t))


Script_entry_state = 0 # ew, magic number, get pgen to emit this


def parse_Script(line):
    return parse(
        es_parser.actions,
        es_parser.ctns,
        es_parser.reductions,
        Script_entry_state,
        line,
        es_parser.DefaultBuilder()
    )

def main():
    while True:
        try:
            line = input('> ')
        except EOFError as _:
            break
        try:
            result = parse_Script(line)
        except SyntaxError as exc:
            print(exc.__class__.__name__ + ": " + str(exc))
        else:
            print(result)

if __name__ == '__main__':
    main()
