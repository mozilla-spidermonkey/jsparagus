//! Functions to exercise the parser from the command line.

use crate::parser_generated;

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::prelude::*;  // flush() at least
use std::path::Path;

use crate::lexer::Lexer;

fn parse_file(path: &Path) -> io::Result<()> {
    print!("{}:", path.display());
    io::stdout().flush()?;
    let contents = match fs::read_to_string(path) {
        Err(err) => {
            println!(" error reading file: {}", err);
            return Ok(());
        }
        Ok(s) => s,
    };
    let result = parser_generated::parse_Script(
        &mut parser_generated::DefaultHandler {},
        Lexer::new(contents.chars())
    );
    match result {
        Ok(_ast) => println!(" ok"),
        Err(err) => println!(" error: {}", err.message()),
    }
    Ok(())
}

pub fn parse_file_or_dir(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let file = entry?.path();
            if file.is_file() && file.extension() == Some(OsStr::new("js")) {
                parse_file(&file)?;
            }
        }
    } else {
        parse_file(Path::new(filename))?;
    }
    Ok(())
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

fn get_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    if let 0 = io::stdin().read_line(&mut input)? {
        Err("EOF".into())
    } else {
        Ok(input.trim().to_string())
    }
}

pub fn read_print_loop() {
    while let Ok(buffer) = get_input("> ") {
        run(&buffer);
    }
}
