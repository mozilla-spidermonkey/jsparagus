#![cfg_attr(feature = "unstable", feature(test))]

extern crate generated_parser;

mod lexer;
mod parser;

#[cfg(all(feature = "unstable", test))]
mod tests;

use lexer::Lexer;
use parser::{ParseError, Parser};

use generated_parser::concrete::{Module, Script};
use generated_parser::{
    get_result_module, get_result_script, DefaultHandler,
    TerminalId, START_STATE_MODULE, START_STATE_SCRIPT, TABLES,
};

pub type Result<T> = std::result::Result<T, ParseError>;

pub fn parse_script(source: &str) -> Result<Script> {
    Ok(get_result_script(parse(source, START_STATE_SCRIPT)?))
}

pub fn parse_module(source: &str) -> Result<Module> {
    Ok(get_result_module(parse(source, START_STATE_MODULE)?))
}

fn parse(source: &str, start_state: usize) -> Result<*mut ()> {
    let mut tokens = Lexer::new(source.chars());

    TABLES.check();

    let mut parser = Parser::new(DefaultHandler {}, start_state);

    loop {
        if let Some(t) = tokens.next(&parser) {
            if t.terminal_id == TerminalId::End {
                break;
            }
            parser.write_token(&t)?;
        } else {
            return Err(ParseError::LexerError);
        }
    }
    parser.close()
}
