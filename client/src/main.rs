#![cfg_attr(feature = "unstable", feature(test))]

mod lexer;
mod parser;
mod parser_generated;
mod parser_runtime;

use crate::lexer::Lexer;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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
            crate::parser_generated::parse_Script(lexer).unwrap();
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        run(&contents);
        return;
    }
    while let Ok(buffer) = get_input("> ") {
        run(&buffer);
    }
}

fn run(buffer: &str) {
    let lexer = Lexer::new(buffer.chars());
    let parse_result =
        parser_generated::parse_Script(&mut parser_generated::DefaultHandler {}, lexer);
    match parse_result {
        Ok(ast) => println!("{:#?}", ast),
        Err(err) => println!("{}", err.message()),
    }
}

pub fn get_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    if let 0 = io::stdin().read_line(&mut input)? {
        Err("EOF".into())
    } else {
        Ok(input.trim().to_string())
    }
}

// fn main() -> io::Result<()> {
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer)?;
//     let lexer = Lexer::new(buffer.chars());
//     parser_generated::parse_Script(&mut parser_generated::DefaultHandler {}, lexer)
//         .expect("parsing grammar on stdin");
//     Ok(())
// }
