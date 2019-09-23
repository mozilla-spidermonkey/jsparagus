#![cfg_attr(feature = "unstable", feature(test))]

mod lexer;
mod parser;

#[cfg(test)]
mod tests;

use crate::parser::Parser;
use ast::{
    arena,
    types::{Module, Script},
};
use bumpalo;
use generated_parser::{
    AstBuilder, StackValue, TerminalId, START_STATE_MODULE, START_STATE_SCRIPT,
    TABLES,
};
pub use generated_parser::{ParseError, Result};
use lexer::Lexer;
use std::io::{self, Write};

pub fn parse_script<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    source: &'alloc str,
) -> Result<'alloc, arena::Box<'alloc, Script<'alloc>>> {
    Ok(parse(allocator, source, START_STATE_SCRIPT)?.to_ast())
}

pub fn parse_module<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    source: &'alloc str,
) -> Result<'alloc, arena::Box<'alloc, Module<'alloc>>> {
    Ok(parse(allocator, source, START_STATE_MODULE)?.to_ast())
}

fn parse<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    source: &'alloc str,
    start_state: usize,
) -> Result<'alloc, StackValue<'alloc>> {
    let mut tokens = Lexer::new(allocator, source.chars());

    TABLES.check();

    let mut parser = Parser::new(AstBuilder { allocator }, start_state);

    loop {
        let t = tokens.next(&parser)?;
        if t.terminal_id == TerminalId::End {
            break;
        }
        parser.write_token(&t)?;
    }
    parser.close()
}

/// Prompt the user for some JS code and read a script from stdin.
/// Returns the parsed script.
///
/// Errors can be `ParseError::UnexpectedEnd` if the user typed Ctrl-D,
/// `ParseError::IOError` if reading stdin or writing stdout failed, or
/// any other `ParseError` if the input isn't valid JS.
pub fn read_script_interactively<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    prompt: &str,
    continue_prompt: &str,
) -> Result<'alloc, arena::Box<'alloc, Script<'alloc>>> {
    TABLES.check();

    let mut parser = Parser::new(AstBuilder { allocator }, START_STATE_SCRIPT);

    print!("{}", prompt);
    loop {
        let mut line = String::new();
        io::stdout().flush()?;
        if io::stdin().read_line(&mut line)? == 0 {
            return Err(ParseError::UnexpectedEnd);
        }
        let line_str: &'alloc str = arena::alloc_str(allocator, &line);

        let mut tokens = Lexer::new(allocator, line_str.chars());
        loop {
            let t = tokens.next(&parser)?;
            if t.terminal_id == TerminalId::End {
                break;
            }
            parser.write_token(&t)?;
        }
        if parser.can_close() {
            break;
        }
        print!("{}", continue_prompt);
    }
    match parser.close()? {
        StackValue::Script(s) => Ok(s),
        other => {
            // Can't happen due to invariants provided by the parser generator;
            // the only finish state reachable from START_STATE_SCRIPT produces
            // a Script.
            panic!("unexpected StackValue: {:?}", other);
        }
    }
}
