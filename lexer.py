""" Lexical analysis is the breaking of a string into tokens. """

import re

class LexicalGrammar:
    def __init__(self, tokens, ignore=r'[ \t]*', **regexps):
        token_list = sorted(tokens.split(), key=len, reverse=True)
        self.ignore_re = re.compile(ignore)
        self.token_re = re.compile("|".join(re.escape(token) for token in token_list))
        self.parser_pairs = [(k, re.compile(v)) for k, v in regexps.items()]

    def __call__(self, source):
        return Tokenizer(self.ignore_re, self.token_re, self.parser_pairs, source)


class Tokenizer:
    def __init__(self, ignore_re, token_re, parser_pairs, source):
        self.ignore_re = ignore_re
        self.token_re = token_re
        self.parser_pairs = parser_pairs
        self.src = source
        self.point = 0

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


