use std::iter;

use crate::lexer::Lexer;
use crate::parse_script;
use crate::parser::Parser;
use ast::{arena, source_location::SourceLocation, types::*};
use bumpalo::{self, Bump};
use generated_parser::{self, AstBuilder, ParseError, Result, TerminalId};

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

fn try_parse<'alloc, 'source, Source>(
    allocator: &'alloc Bump,
    code: Source,
) -> Result<arena::Box<'alloc, Script<'alloc>>>
where
    Source: IntoChunks<'source>,
{
    let buf = arena::alloc_str(allocator, &chunks_to_string(code));
    parse_script(allocator, &buf)
}

fn assert_parses<'alloc, T: IntoChunks<'alloc>>(code: T) {
    let allocator = &Bump::new();
    try_parse(allocator, code).unwrap();
}

fn assert_syntax_error<'alloc, T: IntoChunks<'alloc>>(code: T) {
    let allocator = &Bump::new();
    assert!(match try_parse(allocator, code) {
        Err(ParseError::SyntaxError(_)) => true,
        Err(other) => panic!("unexpected error: {:?}", other),
        Ok(ast) => panic!("assertion failed: SUCCESS error: {:?}", ast),
    });
}

fn assert_not_implemented<'alloc, T: IntoChunks<'alloc>>(code: T) {
    let allocator = &Bump::new();
    assert!(match try_parse(allocator, code) {
        Err(ParseError::NotImplemented(_)) => true,
        Err(other) => panic!("unexpected error: {:?}", other),
        Ok(ast) => panic!("assertion failed: SUCCESS error: {:?}", ast),
    });
}

fn assert_illegal_character<'alloc, T: IntoChunks<'alloc>>(code: T) {
    let allocator = &Bump::new();
    assert!(match try_parse(allocator, code) {
        Err(ParseError::IllegalCharacter(_)) => true,
        Err(other) => panic!("unexpected error: {:?}", other),
        Ok(ast) => panic!("assertion failed: SUCCESS error: {:?}", ast),
    });
}

fn assert_error_eq<'alloc, T: IntoChunks<'alloc>>(code: T, expected: ParseError) {
    let allocator = &Bump::new();
    let result = try_parse(allocator, code);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), expected);
}

fn assert_incomplete<'alloc, T: IntoChunks<'alloc>>(code: T) {
    let allocator = &Bump::new();
    let result = try_parse(allocator, code);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParseError::UnexpectedEnd);
}

// Assert that `left` and `right`, when parsed as ES Modules, consist of the
// same sequence of tokens (although possibly at different offsets).
fn assert_same_tokens<'alloc>(left: &str, right: &str) {
    let allocator = &Bump::new();
    let mut left_lexer = Lexer::new(allocator, left.chars());
    let mut right_lexer = Lexer::new(allocator, right.chars());

    let mut parser = Parser::new(
        AstBuilder { allocator },
        generated_parser::START_STATE_MODULE,
    );

    loop {
        let left_token = left_lexer.next(&parser).expect("error parsing left string");
        let right_token = right_lexer
            .next(&parser)
            .expect("error parsing right string");
        assert_eq!(
            left_token.terminal_id, right_token.terminal_id,
            "at offset {} in {:?} / {} in {:?}",
            left_token.loc.start, left, right_token.loc.start, right,
        );
        assert_eq!(
            left_token.value, right_token.value,
            "at offsets {} / {}",
            left_token.loc.start, right_token.loc.start
        );

        if left_token.terminal_id == TerminalId::End {
            break;
        }
        parser.write_token(&left_token).unwrap();
    }
    parser.close(left_lexer.offset()).unwrap();
}

fn assert_can_close_after<'alloc, T: IntoChunks<'alloc>>(code: T) {
    let allocator = &Bump::new();
    let buf = chunks_to_string(code);
    let mut lexer = Lexer::new(allocator, buf.chars());
    let mut parser = Parser::new(
        AstBuilder { allocator },
        generated_parser::START_STATE_SCRIPT,
    );
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

    // "[T]he presence or absence of single-line comments does not affect the
    // process of automatic semicolon insertion[...]."
    // <https://tc39.es/ecma262/#sec-comments>
    assert_parses("x = 1 // line break here\ny = 2");
    assert_parses("x = 1 // line break here\r\ny = 2");
    assert_parses("x = 1 /* no line break in here */ //\ny = 2");
    assert_parses("x = 1<!-- line break here\ny = 2");

    assert_syntax_error("x = 1 /* no line break in here */ y = 2");
    assert_parses("x = 1 /* line break \n there */y = 2");
}

#[test]
fn test_asi_suppressed() {
    // The specification says ASI does not happen in the production
    // EmptyStatement : `;`.
    // TODO - assert_syntax_error("if (true)");
    assert_syntax_error("{ for (;;) }");

    // ASI does not happen in for(;;) loops.
    assert_syntax_error("for ( \n ; ) {}");
    assert_syntax_error("for ( ; \n ) {}");
    assert_syntax_error("for ( \n \n ) {}");
    assert_syntax_error("for (var i = 0 \n i < 9;   i++) {}");
    assert_syntax_error("for (var i = 0;   i < 9 \n i++) {}");
    assert_syntax_error("for (i = 0     \n i < 9;   i++) {}");
    assert_syntax_error("for (i = 0;       i < 9 \n i++) {}");
    assert_syntax_error("for (let i = 0 \n i < 9;   i++) {}");

    // ASI is suppressed in the production ClassElement[Yield, Await] : `;`
    // to prevent an infinite loop of ASI. lol
    assert_syntax_error("class Fail { \n +1; }");
}

#[test]
fn test_if_else() {
    assert_parses("if (x) f();");
    assert_incomplete("if (x)");
    assert_parses("if (x) f(); else g();");
    assert_incomplete("if (x) f(); else");
    assert_parses("if (x) if (y) g(); else h();");
    assert_parses("if (x) if (y) g(); else h(); else j();");
}

#[test]
fn test_lexer_decimal() {
    assert_parses("0.");
    assert_parses(".5");
    assert_syntax_error(".");
}

#[test]
fn test_numbers() {
    assert_parses("0");
    assert_parses("1");
    assert_parses("10");

    assert_error_eq("0a", ParseError::IllegalCharacter('a'));
    assert_error_eq("1a", ParseError::IllegalCharacter('a'));

    assert_error_eq("1.0a", ParseError::IllegalCharacter('a'));
    assert_error_eq(".0a", ParseError::IllegalCharacter('a'));
    assert_error_eq("1.a", ParseError::IllegalCharacter('a'));

    assert_parses("1.0");
    assert_parses("1.");
    assert_parses("0.");

    assert_parses("1.0e0");
    assert_parses("1.e0");
    assert_parses(".0e0");

    assert_parses("1.0e+0");
    assert_parses("1.e+0");
    assert_parses(".0e+0");

    assert_parses("1.0e-0");
    assert_parses("1.e-0");
    assert_parses(".0e-0");

    assert_error_eq("1.0e", ParseError::UnexpectedEnd);
    assert_error_eq("1.e", ParseError::UnexpectedEnd);
    assert_error_eq(".0e", ParseError::UnexpectedEnd);

    assert_error_eq("1.0e+", ParseError::UnexpectedEnd);
    assert_error_eq("1.0e-", ParseError::UnexpectedEnd);
    assert_error_eq(".0e+", ParseError::UnexpectedEnd);
    assert_error_eq(".0e-", ParseError::UnexpectedEnd);

    assert_parses(".0");
    assert_parses("");

    // FIXME: NYI: non-decimal literal
    // assert_parses("0b0");
    assert_not_implemented("0b0");

    /*
    assert_parses("0b1");
    assert_parses("0B01");
    assert_error_eq("0b", ParseError::UnexpectedEnd);
    assert_error_eq("0b ", ParseError::IllegalCharacter(' '));
    assert_error_eq("0b2", ParseError::IllegalCharacter('2'));

    assert_parses("0o0");
    assert_parses("0o7");
    assert_parses("0O01234567");
    assert_error_eq("0o", ParseError::UnexpectedEnd);
    assert_error_eq("0o ", ParseError::IllegalCharacter(' '));
    assert_error_eq("0o8", ParseError::IllegalCharacter('8'));

    assert_parses("0x0");
    assert_parses("0xf");
    assert_parses("0X0123456789abcdef");
    assert_parses("0X0123456789ABCDEF");
    assert_error_eq("0x", ParseError::UnexpectedEnd);
    assert_error_eq("0x ", ParseError::IllegalCharacter(' '));
    assert_error_eq("0xg", ParseError::IllegalCharacter('g'));
     */

    assert_parses("1..x");
}

#[test]
fn test_arrow() {
    assert_parses("x => x");
    assert_parses("f = x => x;");
    assert_parses("(x, y) => [y, x]");
    assert_parses("f = (x, y) => {}");
    // XXX TODO
    // assert_syntax_error("(x, y) => {x: x, y: y}");
}

#[test]
fn test_illegal_character() {
    assert_illegal_character("\0");
    assert_illegal_character("—x;");
    assert_illegal_character("const ONE_THIRD = 1 ÷ 3;");
}

#[test]
fn test_identifier() {
    // U+00B7 MIDDLE DOT is an IdentifierPart.
    // assert_parses("_·_ = {_·_:'·_·'};");  // would fail

    // <ZWJ> and <ZWNJ> match IdentifierPart but not IdentifierStart.
    assert_parses("var x\u{200c};"); // <ZWNJ>
    assert_parses("_\u{200d}();"); // <ZWJ>
    assert_parses("_\u{200d}__();"); // <ZWJ>
    assert_parses("_\u{200d}\u{200c}();"); // <ZWJ>
    assert_illegal_character("var \u{200c};"); // <ZWNJ>
    assert_illegal_character("x = \u{200d};"); // <ZWJ>
}

#[test]
fn test_regexp() {
    assert_parses(r"/\w/");
    assert_parses("/[A-Z]/");
    assert_parses("/[//]/");
    assert_parses("/a*a/");
    assert_parses("/**//x*/");
    assert_same_tokens("/**//x*/", "/x*/");
    assert_parses("{} /x/");
    assert_parses("of / 2");
}

#[test]
fn test_html_comments() {
    assert_same_tokens("x<!--y;", "x");
    assert_same_tokens("x<!-y;", "x < ! - y ;");
    assert_same_tokens("x<!y", "x < ! y");

    assert_same_tokens("--> hello world\nok", "ok");
    assert_same_tokens("/* ignore */ --> also ignore\nok", "ok");
    assert_same_tokens("/* ignore *//**/--> also ignore\nok", "ok");
    assert_same_tokens("x-->y\nz", "x -- > y\nz");
}

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
fn test_strings() {
    assert_parses("f(\"\",\"\")");
    assert_parses("f(\"\")");
    assert_parses("(\"\")");
    assert_parses("f('','')");
    assert_parses("f('')");
    assert_parses("('')");
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

    let allocator = &Bump::new();
    let actual = try_parse(allocator, &vec!["x/", "=2;"]).unwrap();
    let expected = Script {
        directives: arena::Vec::new_in(allocator),
        statements: bumpalo::vec![
            in allocator;
            Statement::ExpressionStatement(arena::alloc(
                allocator,
                Expression::CompoundAssignmentExpression {
                    operator: CompoundAssignmentOperator::Div {
                        loc: SourceLocation::new(1, 3),
                    },
                    binding: SimpleAssignmentTarget::AssignmentTargetIdentifier(
                        AssignmentTargetIdentifier {
                            name: Identifier {
                                value: "x",
                                loc: SourceLocation::new(0, 1),
                            },
                            loc: SourceLocation::new(0, 1),
                        },
                    ),
                    expression: arena::alloc(
                        allocator,
                        Expression::LiteralNumericExpression {
                            value: 2.0,
                            loc: SourceLocation::new(3, 4),
                        },
                    ),
                    loc: SourceLocation::new(0, 4),
                },
            ))
        ],
        loc: SourceLocation::new(0, 4),
    };
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
    assert_error_eq("/x", ParseError::UnterminatedRegExp);
    assert_incomplete("x = //"); // comment
    assert_incomplete("x = /*/"); /*/ comment */
    assert_error_eq("x =/= 2", ParseError::UnterminatedRegExp);
    assert_parses("x /= 2");
    assert_parses("x = /[]/");
    assert_parses("x = /[^x]/");
    assert_parses("x = /+=351*/");
    assert_parses("x = /^\\s*function (\\w+)/;");
    assert_parses("let regexp = /this is fine: [/] dont @ me/;");
}

#[test]
fn test_arrow_parameters() {
    assert_error_eq(
        "({a:a, ...b, c:c}) => {}",
        ParseError::ObjectPatternWithNonFinalRest,
    );
    assert_error_eq(
        "(a, [...zero, one]) => {}",
        ParseError::ArrayPatternWithNonFinalRest,
    );
    assert_error_eq(
        "(a, {items: [...zero, one]}) => {}",
        ParseError::ArrayPatternWithNonFinalRest,
    );
}

#[test]
fn test_invalid_assignment_targets() {
    assert_syntax_error("2 + 2 = x;");
    assert_error_eq("(2 + 2) = x;", ParseError::InvalidAssignmentTarget);
    assert_error_eq("++-x;", ParseError::InvalidAssignmentTarget);
    assert_error_eq("(x && y)--;", ParseError::InvalidAssignmentTarget);
}

// XXX TODO
//#[test]
//fn test_can_close_with_asi() {
//    assert_can_close_after("2 + 2\n");
//}

#[test]
fn test_conditional_keywords() {
    // property names
    assert_parses("let obj = {if: 3, function: 4};");
    assert_parses("assert(obj.if == 3);");

    // method names
    assert_parses(
        "
        class C {
            if() {}
            function() {}
        }
        ",
    );

    assert_parses("var let = [new Date];"); // let as identifier
    assert_parses("let v = let;"); // let as keyword, then identifier

    // Next line would fail because the multitoken `let [` lookahead isn't implemented yet.
    // assert_parses("let.length;");           // `let .` -> ExpressionStatement

    assert_syntax_error("let[0].getYear();"); // `let [` -> LexicalDeclaration

    assert_parses(
        "
        var of = [1, 2, 3];
        for (of of of) console.log(of);  // logs 1, 2, 3
        ",
    );

    assert_parses("var of, let, private, target;");
    assert_parses("class X { get y() {} }");
    assert_parses("async: { break async; }");
    assert_parses("var get = { get get() {}, set get(v) {}, set: 3 };");
    assert_parses("for (async of => {};;) {}");

    // This would fail because this case is currently disabled syntactically.
    // assert_parses("for (async of []) {}");  // would fail
}

#[test]
fn test_async_arrows() {
    assert_parses("let f = async arg => body;");
    assert_parses("f = async (a1, a2) => {};");
    assert_parses("f = async (a1 = b + c, ...a2) => {};");

    assert_error_eq("f = async (a, b + c) => {};", ParseError::InvalidParameter);
    assert_error_eq(
        "f = async (...a1, a2) => {};",
        ParseError::ArrowParametersWithNonFinalRest,
    );

    assert_error_eq("foo(a, b) => {}", ParseError::ArrowHeadInvalid);
    assert_error_eq("obj.async() => {}", ParseError::ArrowHeadInvalid);
}

#[test]
fn test_coalesce() {
    assert_parses("let f = options.prop ?? 0;");
    assert_syntax_error("if (options.prop ?? 0 || options.prop > 1000) {}");
}
