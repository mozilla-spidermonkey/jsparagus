""" Tests for the JS parser. """

import unittest
import jsparagus.lexer
from js_parser.parser import parse_Script, JSParser
from js_parser.lexer import JSLexer


class ESTestCase(unittest.TestCase):
    def parse(self, s):
        if isinstance(s, list):
            f = JSLexer(JSParser())
            for chunk in s:
                f.write(chunk)
            return f.close()
        else:
            return parse_Script(s)

    def assert_parses(self, s):
        self.parse(s)

    def assert_incomplete(self, s):
        """Assert that s fails to parse with UnexpectedEndError.

        (This should be the case if `s` is a prefix of a valid Script.)
        """
        self.assertRaises(jsparagus.lexer.UnexpectedEndError,
                          lambda: parse_Script(s))

    def assert_syntax_error(self, s):
        """Assert that s fails to parse."""
        with self.assertRaises(jsparagus.lexer.SyntaxError):
            parse_Script(s)

    # === Tests!

    def test_asi_at_end(self):
        self.assert_parses("3 + 4")
        self.assert_syntax_error("3 4")
        self.assert_incomplete("3 +")
        self.assert_incomplete("{")
        self.assert_incomplete("{;")

    def disabled_test_asi_at_block_end(self):
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

    def test_asi_suppressed(self):
        # The specification says ASI does not happen in the production
        # EmptyStatement : `;`.
        self.assert_syntax_error("if (true)")
        self.assert_syntax_error("{ for (;;) }")

        # ASI does not happen in for(;;) loops.
        self.assert_syntax_error("for ( \n ; ) {}")
        self.assert_syntax_error("for ( ; \n ) {}")
        self.assert_syntax_error("for ( \n \n ) {}")
        self.assert_syntax_error("for (var i = 0 \n i < 9;   i++) {}")
        self.assert_syntax_error("for (var i = 0;   i < 9 \n i++) {}")
        self.assert_syntax_error("for (i = 0 \n i < 9;   i++) {}")
        self.assert_syntax_error("for (i = 0;   i < 9 \n i++) {}")

        # ASI is suppressed in the production ClassElement[Yield, Await] : `;`
        # to prevent an infinite loop of ASI. lol
        self.assert_syntax_error("class Fail { \n +1; }")

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

    def disabled_test_regexp(self):
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

        self.assertEqual(
            self.parse(["/xyzzy/", "g;"]),
            ('Script',
             ('ScriptBody',
              ('StatementList 0',
               ('ExpressionStatement',
                ('PrimaryExpression 10', '/xyzzy/g'))))))

        self.assertEqual(
            self.parse(['x/', '=2;']),
            ('Script',
             ('ScriptBody',
              ('StatementList 0',
               ('ExpressionStatement',
                ('AssignmentExpression 5',
                 ('PrimaryExpression 1', ('IdentifierReference', 'x')),
                 ('AssignmentOperator 1', '/='),
                 ('Literal 2', '2')))))))


if __name__ == '__main__':
    unittest.main()
