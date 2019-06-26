mod ast;
mod parser_generated;
mod parser_runtime;
mod lexer;

use std::io::{self, Read};
use crate::lexer::Lexer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let lexer = Lexer::new(buffer.chars());
    match parser_generated::parse_grammar(lexer) {
        Ok(node) => println!("{:?}", node),
        Err(exc) => println!("error: {}", exc),
    }
    Ok(())
}
