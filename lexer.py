""" Lexical analysis is the breaking of a string into tokens. """

import re

class LexicalGrammar:
    def __init__(self, tokens, **regexps):
        token_list = sorted(tokens.split(), key=len, reverse=True)
        self.token_re = re.compile("|".join(re.escape(token) for token in token_list))
        self.parser_pairs = [(k, re.compile(v)) for k, v in regexps.items()]

    def __call__(self, source):
        return Tokenizer(self.token_re, self.parser_pairs, source)


class Tokenizer:
    def __init__(self, token_re, parser_pairs, source):
        self.token_re = token_re
        self.parser_pairs = parser_pairs
        self.src = source
        self.point = 0

    def _match(self):
        while self.point < len(self.src) and self.src[self.point] in " \t":
            self.point += 1
        if self.point == len(self.src):
            return None
        match = self.token_re.match(self.src, self.point)
        if match is not None:
            return match.group(0), match
        for name, pattern in self.parser_pairs:
            match = pattern.match(self.src, self.point)
            if match is not None:
                return name, match
        raise ValueError("unexpected characters {!r}".format(self.src[self.point:self.point+4]))

    def peek(self):
        hit = self._match()
        if hit is None:
            return None
        return hit[0]

    def take(self, k):
        hit = self._match()
        if hit is None:
            if k is not None:
                raise ValueError("unexpected end of input (expected {!r})".format(k))
        else:
            name, match = hit
            if k != name:
                raise ValueError("expected {!r}, got {!r}".format(k, name))
            self.point = match.end()
            return match.group()


