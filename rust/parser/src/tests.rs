use std::iter;

use crate::lexer::Lexer;
use crate::parser::{ParseError, Parser};
use crate::{parse_script, Result};
use generated_parser::concrete::Script;
use generated_parser::{self, concrete, DefaultHandler, TerminalId};

#[cfg(all(feature = "unstable", test))]
mod benchmarks {
    extern crate test;

    use std::fs::File;
    use std::io::Read;
    use test::Bencher;

    use crate::lexer::Lexer;
    use crate::parse_script;

    #[bench]
    fn bench_parse_grammar(b: &mut Bencher) {
        let mut buffer = fs::read_to_string("../vue.js").expect("reading test file");
        b.iter(|| {
            let lexer = Lexer::new(buffer.chars());
            parse_script(lexer).unwrap();
        });
    }
}

trait IntoChunks<'a> {
    type Chunks: Iterator<Item = &'a str>;
    fn into_chunks(self) -> Self::Chunks;
}

impl<'a> IntoChunks<'a> for &'a str {
    type Chunks = iter::Once<&'a str>;
    fn into_chunks(self) -> Self::Chunks {
        iter::once(self)
    }
}

impl<'a> IntoChunks<'a> for &'a Vec<&'a str> {
    type Chunks = iter::Cloned<std::slice::Iter<'a, &'a str>>;
    fn into_chunks(self) -> Self::Chunks {
        self.iter().cloned()
    }
}

// Glue all the chunks together.  XXX TODO Once the lexer supports chunks,
// we'll reimplement this to feed the code to the lexer one chunk at a time.
fn chunks_to_string<'a, T: IntoChunks<'a>>(code: T) -> String {
    let mut buf = String::new();
    for chunk in code.into_chunks() {
        buf.push_str(chunk);
    }
    buf
}

fn try_parse<'a, T: IntoChunks<'a>>(code: T) -> Result<Box<Script>> {
    let buf = chunks_to_string(code);
    parse_script(&buf)
}

fn assert_parses<'a, T: IntoChunks<'a>>(code: T) {
    try_parse(code).unwrap();
}

fn assert_syntax_error<'a, T: IntoChunks<'a>>(code: T) {
    assert!(match try_parse(code) {
        Err(ParseError::SyntaxError(_)) => true,
        _ => false,
    });
}

fn assert_lexer_error<'a, T: IntoChunks<'a>>(code: T) {
    let result = try_parse(code);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParseError::LexerError);
}

fn assert_incomplete<'a, T: IntoChunks<'a>>(code: T) {
    let result = try_parse(code);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParseError::UnexpectedEnd);
}

fn assert_can_close_after<'a, T: IntoChunks<'a>>(code: T) {
    let buf = chunks_to_string(code);
    let mut lexer = Lexer::new(buf.chars());
    let mut parser = Parser::new(DefaultHandler {}, generated_parser::START_STATE_SCRIPT);
    loop {
        let t = lexer.next(&parser).expect("lexer error");
        if t.terminal_id == TerminalId::End {
            break;
        }
        parser.write_token(&t).unwrap();
    }
    assert!(parser.can_close());
}

#[test]
fn test_asi_at_end() {
    assert_parses("3 + 4");
    assert_syntax_error("3 4");
    assert_incomplete("3 +");
    assert_incomplete("{");
    assert_incomplete("{;");
}

#[test]
fn test_asi_at_block_end() {
    assert_parses("{ doCrimes() }");
    assert_parses("function f() { ok }");
}

#[test]
fn test_asi_after_line_terminator() {
    assert_parses(
        "switch (value) {
             case 1: break
             case 2: console.log('2');
         }",
    );
    assert_syntax_error("switch (value) { case 1: break case 2: console.log('2'); }");
}

// XXX TODO - uncomment passing assertions, fix bugs in asi support
//#[test]
//fn test_asi_suppressed() {
//    // The specification says ASI does not happen in the production
//    // EmptyStatement : `;`.
//    assert_syntax_error("if (true)");
//    assert_syntax_error("{ for (;;) }");
//
//    // ASI does not happen in for(;;) loops.
//    assert_syntax_error("for ( \n ; ) {}");
//    assert_syntax_error("for ( ; \n ) {}");
//    assert_syntax_error("for ( \n \n ) {}");
//    assert_syntax_error("for (var i = 0 \n i < 9;   i++) {}");
//    assert_syntax_error("for (var i = 0;   i < 9 \n i++) {}");
//    assert_syntax_error("for (i = 0     \n i < 9;   i++) {}");
//    assert_syntax_error("for (i = 0;       i < 9 \n i++) {}");
//    assert_syntax_error("for (let i = 0 \n i < 9;   i++) {}");
//
//    // ASI is suppressed in the production ClassElement[Yield, Await] : `;`
//    // to prevent an infinite loop of ASI. lol
//    assert_syntax_error("class Fail { \n +1; }");
//}

#[test]
fn test_if_else() {
    assert_parses("if (x) f();");
    assert_incomplete("if (x)");
    assert_parses("if (x) f(); else g();");
    assert_incomplete("if (x) f(); else");
    assert_parses("if (x) if (y) g(); else h();");
    assert_parses("if (x) if (y) g(); else h(); else j();");
}

// XXX TODO
//#[test]
//fn test_lexer_decimal() {
//    assert_parses("0.");
//    assert_parses(".5");
//    assert_syntax_error(".");
//}

#[test]
fn test_arrow() {
    assert_parses("x => x");
    assert_parses("f = x => x;");
    assert_parses("(x, y) => [y, x]");
    assert_parses("f = (x, y) => {}");
    assert_syntax_error("(x, y) => {x: x, y: y}");
}

// XXX TODO
//#[test]
//fn test_invalid_character() {
//    assert_syntax_error("\0");
//    assert_syntax_error("—x;");
//    assert_syntax_error("const ONE_THIRD = 1 ÷ 3;");
//}

// XXX TODO
//#[test]
//fn test_regexp() {
//    assert_parses(r"/\w/");
//    assert_parses("/[A-Z]/");
//    assert_parses("/[//]/");
//    assert_parses("/a*a/");
//    assert_parses("/**//x*/");
//    assert_parses("{} /x/");
//    assert_parses("of / 2");
//}

#[test]
fn test_incomplete_comments() {
    // XXX TODO
    // assert_syntax_error("/*");
    // assert_syntax_error("/* hello world");
    // assert_syntax_error("/* hello world *");
    assert_parses(&vec!["/* hello\n", " world */"]);
    assert_parses(&vec!["// oawfeoiawj", "ioawefoawjie"]);
    assert_parses(&vec!["// oawfeoiawj", "ioawefoawjie\n ok();"]);
    assert_parses(&vec!["// oawfeoiawj", "ioawefoawjie", "jiowaeawojefiw"]);
    assert_parses(&vec![
        "// oawfeoiawj",
        "ioawefoawjie",
        "jiowaeawojefiw\n ok();",
    ]);
}

#[test]
fn test_awkward_chunks() {
    assert_parses(&vec!["let", "ter.head = 1;"]);
    assert_parses(&vec!["let", " x = 1;"]);

    // Try feeding one character at a time to the parser.
    let chars: Vec<&str> = "function f() { ok(); }".split("").collect();
    assert_parses(&chars);

    // XXX TODO
    //assertEqual(
    //    self.parse(&vec!["/xyzzy/", "g;"]),
    //    ('Script',
    //     ('ScriptBody',
    //      ('StatementList 0',
    //       ('ExpressionStatement',
    //        ('PrimaryExpression 10', '/xyzzy/g'))))));

    let actual = try_parse(&vec!["x/", "=2;"]).unwrap();
    let expected = concrete::Script::Script(Some(Box::new(concrete::ScriptBody::ScriptBody(
        Box::new(concrete::StatementList::StatementListP0(Box::new(
            concrete::ModuleItem::ExpressionStatement(Box::new(
                concrete::Expression::AssignmentExpressionP5(
                    Box::new(concrete::Expression::PrimaryExpressionP1(Box::new(
                        concrete::IdentifierReference::IdentifierReference(),
                    ))),
                    Box::new(concrete::AssignmentOperator::AssignmentOperatorP1()),
                    Box::new(concrete::Expression::LiteralP2()),
                ),
            )),
        ))),
    ))));
    assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
}

#[test]
fn test_can_close() {
    let empty: Vec<&str> = vec![];
    assert_can_close_after(&empty);
    assert_can_close_after("");
    assert_can_close_after("2 + 2;\n");
    assert_can_close_after("// seems ok\n");
}

#[test]
fn test_regex() {
    assert_parses("/x/");
    assert_parses("x = /x/");
    assert_parses("x = /x/g");
    assert_parses("x = /x/wow_flags_can_be_$$anything$$");
    // TODO: Should the lexer running out of input throw an incomplete error, or a lexer error?
    assert_lexer_error("/x");
    assert_incomplete("x = //"); // comment
    assert_incomplete("x = /*/"); /*/ comment */
    assert_lexer_error("x =/= 2"); // unterminated regex
    assert_parses("x /= 2");
    assert_parses("x = /[]/");
    assert_parses("x = /[^x]/");
    assert_parses("x = /+=351*/");
}

// XXX TODO
//#[test]
//fn test_can_close_with_asi() {
//    assert_can_close_after("2 + 2\n");
//}
