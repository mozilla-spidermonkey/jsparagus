#![cfg_attr(feature = "unstable", feature(test))]

mod ast;
mod lexer;
mod parser_generated;
mod parser_runtime;

use crate::lexer::Lexer;
use std::io::{self, Read};

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;

    use crate::lexer::Lexer;
    use std::fs::File;
    use std::io::Read;
    use test::Bencher;

    #[bench]
    fn bench_parse_grammar(b: &mut Bencher) {
        let mut file = File::open("../mtg/mtg.pgen").expect("opening test file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("reading from test file");
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
