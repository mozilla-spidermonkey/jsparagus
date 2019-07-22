""" Tests for the JS parser. """

import unittest
import jsparagus.lexer
from js_parser.parser import parse_Script, JSParser
from js_parser.lexer import JSLexer


class ESTestCase(unittest.TestCase):
    def assert_parses(self, s):
        if isinstance(s, list):
            f = JSLexer(JSParser())
            for chunk in s:
                f.write(chunk)
            f.close()
        else:
            parse_Script(s)

    def assert_incomplete(self, s):
        """Assert that s fails to parse with UnexpectedEndError.

        (This should be the case if `s` is a prefix of a valid Script.)
        """
        self.assertRaises(jsparagus.lexer.UnexpectedEndError,
                          lambda: parse_Script(s))

    def assert_syntax_error(self, s):
        """Assert that s fails to parse."""
        self.assertRaises(jsparagus.lexer.SyntaxError,
                          lambda: parse_Script(s))

    # === Tests!

    def test_asi_at_end(self):
        self.assert_parses("3 + 4")
        self.assert_syntax_error("3 4")
        self.assert_incomplete("3 +")
        self.assert_incomplete("{")
        self.assert_incomplete("{;")

    def test_asi_at_block_end(self):
        self.assert_parses("{ doCrimes() }")
        self.assert_parses("function f() { ok }")

    def test_asi_after_line_terminator(self):
        self.assert_parses('''\
           switch (value) {
             case 1: break
             case 2: console.log('2');
           }
        ''')
        self.assert_syntax_error(
            "switch (value) { case 1: break case 2: console.log('2'); }")

    def test_if_else(self):
        self.assert_parses("if (x) f();")
        self.assert_incomplete("if (x)")
        self.assert_parses("if (x) f(); else g();")
        self.assert_incomplete("if (x) f(); else")
        self.assert_parses("if (x) if (y) g(); else h();")
        self.assert_parses("if (x) if (y) g(); else h(); else j();")

    def test_lexer_decimal(self):
        self.assert_parses("0.")
        self.assert_parses(".5")
        self.assert_syntax_error(".")

    def test_arrow(self):
        self.assert_parses("x => x")
        self.assert_parses("f = x => x;")
        self.assert_parses("(x, y) => [y, x]")
        self.assert_parses("f = (x, y) => {}")
        self.assert_syntax_error("(x, y) => {x: x, y: y}")

    def test_invalid_character(self):
        self.assert_syntax_error("\0")
        self.assert_syntax_error("—x;")
        self.assert_syntax_error("const ONE_THIRD = 1 ÷ 3;")

    def test_regexp(self):
        self.assert_parses(r"/\w/")
        self.assert_parses("/[A-Z]/")
        self.assert_parses("/[//]/")
        self.assert_parses("/a*a/")
        self.assert_parses("/**//x*/")
        self.assert_parses("{} /x/")
        self.assert_parses("of / 2")

    def test_incomplete_comments(self):
        self.assert_syntax_error("/*")
        self.assert_syntax_error("/* hello world")
        self.assert_syntax_error("/* hello world *")
        self.assert_parses(["/* hello\n", " world */"])
        self.assert_parses(["// oawfeoiawj", "ioawefoawjie"])
        self.assert_parses(["// oawfeoiawj", "ioawefoawjie\n ok();"])
        self.assert_parses(["// oawfeoiawj", "ioawefoawjie", "jiowaeawojefiw"])
        self.assert_parses(["// oawfeoiawj", "ioawefoawjie", "jiowaeawojefiw\n ok();"])

    def test_awkward_chunks(self):
        self.assert_parses(["let", "ter.head = 1;"])
        self.assert_parses(["let", " x = 1;"])

        # `list()` here explodes the string into a list of one-character strings.
        self.assert_parses(list("function f() { ok(); }"))

if __name__ == '__main__':
    unittest.main()
