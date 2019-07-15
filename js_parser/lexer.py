"""Vague approximation of an ECMAScript lexer."""

import re
import jsparagus.lexer


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
    | \.\.\. | \. | <<=? | <= | < | >>>=? | >>=? | >= | >
    | === | == | = | !== | != | !
    | \+\+ | \+=? | -- | -=? | \*\*=? | \*=? | %=?
    | && | &=? | \^=? | \|\| | \|=? | =>
      # The slash special case
    | /
      # The curly brace special case
    | }
      # StringLiteral
    | ' (?: [^'\\\r\n] | \\['"] | \\x[0-9A-Fa-f]{2}
         | \\u[0-9A-Fa-f]{4} | \\u\{[0-9A-Fa-f]+\}
         | \\\r\n? | \\[\n\u2028\u2029] )* ' # TODO finish list of escapes
    | " (?: [^"\\\r\n] | \\['"] | \\x[0-9A-Fa-f]{2}
         | \\u[0-9A-Fa-f]{4} | \\u\{[0-9A-Fa-f]+\}
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


class JSLexer(jsparagus.lexer.BaseLexer):
    """Vague approximation of an ECMAScript lexer. """
    def __init__(self, source, parser, filename=None):
        self.src = source
        self.filename = filename
        self.last_point = 0
        self.point = 0
        self._next_kind = None
        self.parser = parser

    def _match(self):
        match = self._next_match = TOKEN_RE.match(self.src, self.point)
        assert match is not None, "TOKEN_RE should always match"
        token = match.group(1)
        self.point = match.start(1)

        if token == '':
            assert match.end() == len(self.src)
            # Implement ASI at the end of the program.
            if (not self.parser.can_accept(None) and
                    self.parser.can_accept(';') and
                    not self.parser.can_accept_EmptyStatement()):
                return ';'
            return None
        c = token[0]
        if c.isdigit():
            return 'NumericLiteral'
        elif c.isalpha() or c in '$_':
            if self.parser.can_accept('IdentifierName'):
                return 'IdentifierName'
            elif token in RESERVED_WORDS:  # TODO support strict mode
                if token == 'null':
                    return 'NullLiteral'
                elif token in ('true', 'false'):
                    return 'BooleanLiteral'
                return token
            elif (token in ('let', 'static', 'yield', 'async', 'of') and
                  self.parser.can_accept(token)):
                # This is not what the standard says but eh
                return token
            else:
                return 'Identifier'
        elif c == '/':
            if self.parser.can_accept('RegularExpressionLiteral'):
                raise Exception("not supported: regular expression literals")
            else:
                match = re.match(r'(/=?)', self.src, self.point)
                self._next_match = match
                token = match.group(1)
            return token
        elif c == '`':
            if token.endswith('`'):
                return 'NoSubstitutionTemplate'
            else:
                return 'TemplateHead'
        elif c == '"' or c == "'":
            return 'StringLiteral'
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

    def last_point_coords(self):
        src_pre = self.src[:self.last_point]
        lineno = 1 + src_pre.count("\n")
        line_start_index = src_pre.rfind("\n") + 1
        column = self.last_point - line_start_index  # can be zero
        return lineno, column
