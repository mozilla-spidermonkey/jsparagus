""" Tests for the JS parser. """

from .context import jsparagus
import unittest
import js_parser.parser

class ESTestCase(unittest.TestCase):
    def assert_parses(self, s):
        js_parser.parser.parse_Script(s)

    def assert_incomplete(self, s):
        """Assert that s fails to parse with UnexpectedEndError.

        (This should be the case if `s` is a prefix of a valid Script.)
        """
        self.assertRaises(jsparagus.lexer.UnexpectedEndError,
                          lambda: js_parser.parser.parse_Script(s))

    def assert_syntax_error(self, s):
        """Assert that s fails to parse."""
        self.assertRaises(jsparagus.lexer.SyntaxError,
                          lambda: js_parser.parser.parse_Script(s))

    # === Tests!

    def test_asi_at_end(self):
        self.assert_parses("3 + 4")
        self.assert_syntax_error("3 4")
        self.assert_incomplete("3 +")
        self.assert_incomplete("{")
        self.assert_incomplete("{;")

    def test_if_else(self):
        self.assert_parses("if (x) f();")
        self.assert_incomplete("if (x)")
        self.assert_parses("if (x) f(); else g();")
        self.assert_incomplete("if (x) f(); else")
        self.assert_parses("if (x) if (y) g(); else h();")
        self.assert_parses("if (x) if (y) g(); else h(); else j();")

if __name__ == '__main__':
    unittest.main()
