extern crate test;

use crate::lexer::Lexer;
use std::fs::File;
use std::io::Read;
use test::Bencher;

#[bench]
fn bench_parse_grammar(b: &mut Bencher) {
    let mut buffer = fs::read_to_string("../vue.js").expect("reading test file");
    b.iter(|| {
        let lexer = Lexer::new(buffer.chars());
        crate::parser_generated::parse_Script(lexer).unwrap();
    });
}
