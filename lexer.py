""" Lexical analysis is the breaking of a string into tokens. """

import re


class LexicalGrammar:
    def __init__(self, tokens, ignore=r'[ \t]*', **regexps):
        token_list = sorted(tokens.split(), key=len, reverse=True)
        self.ignore_re = re.compile(ignore)
        self.token_re = re.compile("|".join(re.escape(token) for token in token_list))
        self.parser_pairs = [(k, re.compile(v)) for k, v in regexps.items()]

    def __call__(self, source, filename=None):
        return Tokenizer(self.ignore_re, self.token_re, self.parser_pairs, source, filename)


class Tokenizer:
    def __init__(self, ignore_re, token_re, parser_pairs, source, filename=None):
        self.ignore_re = ignore_re
        self.token_re = token_re
        self.parser_pairs = parser_pairs
        self.src = source
        self.filename = filename
        self.last_point = 0
        self.point = 0
        self._next_match = None
        self._next_kind = None

    def _match(self):
        ignore_match = self.ignore_re.match(self.src, self.point)
        if ignore_match is not None:
            self.point = ignore_match.end()

        if self.point == len(self.src):
            return None

        # Try the token_re.
        token_match = self.token_re.match(self.src, self.point)

        # Try all the parser_pairs.
        for name, pattern in self.parser_pairs:
            match = pattern.match(self.src, self.point)
            if match is not None:
                break
        else:
            name = match = None

        if match is not None and token_match is not None and match.end() > token_match.end():
            return name, match
        elif token_match is not None:
            return token_match.group(0), token_match
        elif match is not None:
            return name, match
        else:
            self.throw("unexpected characters {!r}".format(self.src[self.point:self.point+12]))

    def peek(self):
        if self._next_kind is not None:
            return self._next_kind
        hit = self._match()
        if hit is None:
            return None
        self._next_kind, self._next_match = hit
        return self._next_kind

    def take(self, k):
        next_kind = self.peek()
        if next_kind is None:
            if k is not None:
                self.throw("unexpected end of input (expected {!r})".format(k))
        else:
            if k != next_kind:
                self.throw("expected {!r}, got {!r}".format(k, next_kind))
            match = self._next_match
            self.last_point = match.start()
            self.point = match.end()
            self._next_kind = None
            self._next_match = None
            return match.group()

    def last_point_coords(self):
        src_pre = self.src[:self.last_point]
        lines = src_pre.splitlines()
        lineno = len(lines)
        if lineno == 0:
            lineno = 1
            column = 0
        else:
            column = len(lines[-1])
        return lineno, column

    def throw(self, msg):
        e = SyntaxError(msg)
        e.filename = self.filename
        e.lineno, e.column = self.last_point_coords()
        raise e
