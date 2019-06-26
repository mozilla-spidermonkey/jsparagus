#![feature(test)]

extern crate test;

mod ast;
mod parser_generated;
mod parser_runtime;
mod lexer;

use std::io::{self, Read};
use crate::lexer::Lexer;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use test::Bencher;
    use crate::lexer::Lexer;

    #[bench]
    fn bench_parse_grammar(b: &mut Bencher) {
        let mut file = File::open("../mtg/mtg.pgen").expect("opening test file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("reading from test file");
        b.iter(|| {
            let lexer = Lexer::new(buffer.chars());
            crate::parser_generated::parse_grammar(lexer).unwrap();
        });
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let lexer = Lexer::new(buffer.chars());
    parser_generated::parse_grammar(lexer).expect("parsing grammar on stdin");
    Ok(())
}
