"""Vague approximation of an ECMAScript lexer.

A parser has two levels: the *lexer* scans bytes to produce tokens. The
*parser* consumes tokens and produces ASTs.

In a traditional design, the parser drives the process. It *pulls* one token at
a time from the lexer. However, for a parser that can accept arbitrary slabs of
data, scan them, then keep going, it makes more sense for the user to feed
those slabs to the lexer, which then *pushes* tokens to the parser. So that's
what we do.

Usage:

    from js_parser.lexer import JSLexer
    from js_parser.parser import JSParser

    lexer = JSLexer(JSParser())
    lexer.write(some_source_text)
    lexer.write(some_more_source_text)
    ast = lexer.close()
"""

import re
import jsparagus.lexer


def _get_punctuators():
    punctuators = '''
        { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ --
        << >> >>> & | ^ ! ~ && || ? : = += -= *= %=
        **= ><<= >>= >>>= &= |= ^= =>
    '''.split()

    return '|'.join(
        re.escape(token)
        for token in sorted(punctuators, key=len, reverse=True))


TOKEN_RE = re.compile(r'''(?x)
  (?:
      # WhiteSpace
      [\ \t\v\r\n\u00a0\u2028\u2029\ufeff]
      # SingleLineComment
    | // [^\r\n\u2028\u2029]* (?= [\r\n\u2028\u2029] | \Z )
      # MultiLineComment
    | /\*  (?: [^*] | \*+[^/] )*  \*+/
  )*
  (
      # Incomplete MultiLineComment
      /\*  (?: [^*] | \*+[^/] )*  \**
    | # Incomplete SingleLineComment
      // [^\r\n\u2028\u2029]*
    | # IdentifierName
      (?: [$_A-Za-z]     | \\ u [0-9A-Fa-f]{4} | \\ u \{ [0-9A-Fa-f]+ \})
      (?: [$_0-9A-Za-z]  | \\ u [0-9A-Fa-f]{4} | \\ u \{ [0-9A-Fa-f]+ \})*
    | # NumericLiteral
      [0-9][0-9A-Za-z]*(?:\.[0-9A-Za-z]*)?
    | \.[0-9][0-9A-Za-z]*
    | # Punctuator
      <INSERT_PUNCTUATORS>
    | # The slash special case
      /
    | # The curly brace special case
      }
    | # StringLiteral
      '
        # SingleStringCharacters
        (?:
            # SourceCharacter but not one of ' or \\ or LineTerminator
            # but also allow LINE SEPARATOR or PARAGRAPH SEPARATOR
            [^'\\\r\n]
          | \\ [^0-9xu\r\n\u2028\u2029]  # CharacterEscapeSequence
          | \\ x [0-9A-Fa-f]{2}          # HexEscapeSequence
          | \\ u [0-9A-Fa-f]{4}          # UnicodeEscapeSequence
          | \\ u \{ [0-9A-Fa-f]+ \}
          | \\\r\n?                      # LineContinuation
          | \\[\n\u2028\u2029]
        )*
      '
    | "
        # DoubleStringCharacters
        (?:
            # SourceCharacter but not one of " or \\ or LineTerminator
            # but also allow LINE SEPARATOR or PARAGRAPH SEPARATOR
            [^"\\\r\n]
          | \\ [^0-9xu\r\n\u2028\u2029]  # CharacterEscapeSequence
          | \\ x [0-9A-Fa-f]{2}          # HexEscapeSequence
          | \\ u [0-9A-Fa-f]{4}          # UnicodeEscapeSequence
          | \\ u \{ [0-9A-Fa-f]+ \}
          | \\\r\n?                      # LineContinuation
          | \\[\n\u2028\u2029]
        )*
      "
    | # Template
      ` (?: [^`\\$] | \\. )* (?: \${ | ` )
    | # illegal character or end of input (this branch matches no characters)
  )
'''.replace("<INSERT_PUNCTUATORS>", _get_punctuators()))

DIV_RE = re.compile(r'(/=?)')

REGEXP_RE = re.compile(r'''(?x)
(
    /
    (?:
        # RegularExpressionFirstChar - implemented using
        #     RegularExpressionChars on the theory that we have already
        #     ruled out the possibility of a comment.
        # RegularExpressionChars
        (?:
            # RegularExpressionNonTerminator but not one of \\ or / or [
            [^/\\\[\r\n\u2028\u2029]
          | # RegularExpressionBackslashSequence
            \\ [^\r\n\u2028\u2029]
          | # RegularExpressionClass
            \[
                # RegularExpressionClassChars
                (?:
                    # RegularExpressionNonTerminator but not one of ] or \\
                    [^]\\\r\n\u2028\u2029]
                  | # RegularExpressionBackslashSequence
                    \\ [^\r\n\u2028\u2029]
                )*
            \]
        )+
    )
    /
    (?: \w* )
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
    def __init__(self, parser, filename=None):
        self.parser = parser
        self.filename = filename
        self.src = ''
        self.previous_token_end = 0
        self.current_token_start = 0
        self.point = 0

    def write(self, text):
        self.src += text
        self._drain(closing=False)

    def close(self):
        self._drain(closing=True)
        assert self.src == ''
        return self.parser.close()

    def _drain(self, closing):
        assert self.previous_token_end == 0
        assert self.current_token_start == 0
        assert self.point == 0

        token = self._match(closing)
        while token is not None:
            self.parser.write_terminal(self, token)
            token = self._match(closing)

        self.src = self.src[self.point:]
        self.point = 0
        self.previous_token_end = 0
        self.current_token_start = 0

    def _match(self, closing):
        match = self._next_match = TOKEN_RE.match(self.src, self.point)
        assert match is not None

        if match.end() == len(self.src) and not closing:
            # The current token runs right up against the end of the current
            # chunk of source and thus might continue in the next chunk. Do not
            # move self.point.
            return None

        token = match.group(1)
        if token == '':
            # Whitespace followed by end of input or illegal character.
            if match.end() == len(self.src):
                # End of input. Success!
                assert closing
                self.point = match.end()
                return None
            else:
                c = self.src[match.end()]
                self.throw("unexpected character: {!r}".format(c))

        self.current_token_start = match.start(1)
        self.point = match.start(1)
        c = token[0]
        if c.isdigit() or c == '.' and token != '.':
            return 'NumericLiteral'
        elif c.isalpha() or c in '$_':
            if self.parser.can_accept_terminal('IdentifierName'):
                return 'IdentifierName'
            elif token in RESERVED_WORDS:  # TODO support strict mode
                if token == 'null':
                    return 'NullLiteral'
                elif token in ('true', 'false'):
                    return 'BooleanLiteral'
                return token
            elif (token in ('let', 'static', 'yield', 'async', 'of') and
                  self.parser.can_accept_terminal(token)):
                # This is not what the standard says but eh
                return token
            else:
                return 'Identifier'
        elif c == '/':
            if token.startswith(('/*', '//')):
                # Incomplete comment. (In non-closing mode, this is handled
                # above, immediately after the match.)
                assert match.end() == len(self.src)
                assert closing
                self.point = len(self.src)
                self.throw("incomplete comment at end of source")

            # We choose RegExp vs. division based on what the parser can
            # accept, a literal implementation of the spec.
            #
            # To make this correct in combination with end-of-line ASI, make
            # the parser rewind the lexer one token and ask for it again in
            # that case, so that the lexer asks the can-accept question again.
            if self.parser.can_accept_terminal('RegularExpressionLiteral'):
                match = REGEXP_RE.match(self.src, self.point)
                self._next_match = match
                token = 'RegularExpressionLiteral'
            else:
                match = DIV_RE.match(self.src, self.point)
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
        elif c in '{()[];,~?:.<>=!+-*%&|^':
            return token
        else:
            assert False

    def peek(self):
        raise TypeError("this is not a standard lexer")

    def saw_line_terminator(self):
        """True if there's a LineTerminator before the current token.

        Call this only after having called `.peek()` more recently than
        `.take()`.
        """
        i = self.previous_token_end
        j = self.current_token_start
        ws_between = self.src[i:j]
        return any(c in ws_between for c in '\r\n\u2028\u2029')

    def take(self, k):
        match = self._next_match
        self.point = match.end()
        self._next_match = None
        return match.group(1)

    def last_point_coords(self):
        # TODO - count lines and characters as we go
        src_pre = self.src[:self.current_token_start]
        lineno = 1 + src_pre.count("\n")
        line_start_index = src_pre.rfind("\n") + 1
        column = self.current_token_start - line_start_index  # can be zero
        return lineno, column

    def can_close(self):
        match = TOKEN_RE.match(self.src)
        return match.group(1) == '' and self.parser.can_close()
