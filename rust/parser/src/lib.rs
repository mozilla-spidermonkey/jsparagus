#![cfg_attr(feature = "unstable", feature(test))]

mod errors;
mod lexer;
mod parser;

#[cfg(test)]
mod tests;

pub use crate::errors::{ParseError, Result};
use crate::parser::Parser;
use ast::{
    arena,
    types::{Module, Script},
};
use bumpalo;
use generated_parser::{
    AstBuilder, StackValue, TerminalId, START_STATE_MODULE, START_STATE_SCRIPT, TABLES,
};
use lexer::Lexer;
use std::error::Error;
use std::fmt::{self, Display};
use std::io::{self, Write};
use std::result;

pub fn parse_script<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    source: &'alloc str,
) -> Result<arena::Box<'alloc, Script<'alloc>>> {
    Ok(parse(allocator, source, START_STATE_SCRIPT)?.to_ast())
}

pub fn parse_module<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    source: &'alloc str,
) -> Result<arena::Box<'alloc, Module<'alloc>>> {
    Ok(parse(allocator, source, START_STATE_MODULE)?.to_ast())
}

fn parse<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    source: &'alloc str,
    start_state: usize,
) -> Result<StackValue<'alloc>> {
    let mut tokens = Lexer::new(source.chars());

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

/// Error type returned by `read_script_interactively` when the user types the
/// end-of-file character (usually Ctrl-D).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UserExit;

impl Display for UserExit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "UserExit")
    }
}

impl Error for UserExit {
    fn description(&self) -> &str {
        "UserExit"
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Prompt the user for some JS code and read a script from stdin.
/// Returns the parsed script.
///
/// Errors can be `parser::UserExit` if the user typed Ctrl-D,
/// `std::io::Error` if reading stdin or writing stdout failed, or
/// `parser::errors::ParseError` if the input isn't valid JS.
pub fn read_script_interactively<'alloc>(
    allocator: &'alloc bumpalo::Bump,
    prompt: &str,
    continue_prompt: &str,
) -> std::result::Result<arena::Box<'alloc, Script<'alloc>>, Box<dyn Error>> {
    TABLES.check();

    let mut parser = Parser::new(AstBuilder { allocator }, START_STATE_SCRIPT);

    print!("{}", prompt);
    loop {
        let mut line = String::new();
        io::stdout().flush()?;
        if io::stdin().read_line(&mut line)? == 0 {
            return Err(UserExit.into());
        }
        let line_str: &'alloc str = arena::alloc_str(allocator, &line);

        let mut tokens = Lexer::new(line_str.chars());
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
