#![cfg_attr(feature = "unstable", feature(test))]

extern crate ast;
extern crate generated_parser;

mod errors;
mod lexer;
mod parser;

#[cfg(test)]
mod tests;

pub use crate::errors::{ParseError, Result};
use crate::parser::Parser;
use ast::{Module, Script};
use generated_parser::{
    AstBuilder, StackValue, TerminalId, START_STATE_MODULE, START_STATE_SCRIPT, TABLES,
};
use lexer::Lexer;

pub fn parse_script(source: &str) -> Result<Box<Script>> {
    Ok(parse(source, START_STATE_SCRIPT)?.to_ast())
}

pub fn parse_module(source: &str) -> Result<Box<Module>> {
    Ok(parse(source, START_STATE_MODULE)?.to_ast())
}

fn parse(source: &str, start_state: usize) -> Result<StackValue> {
    let mut tokens = Lexer::new(source.chars());

    TABLES.check();

    let mut parser = Parser::new(AstBuilder {}, start_state);

    loop {
        let t = tokens.next(&parser)?;
        if t.terminal_id == TerminalId::End {
            break;
        }
        parser.write_token(&t)?;
    }
    parser.close()
}
