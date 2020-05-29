//! Functions to exercise the parser from the command line.

use std::cell::RefCell;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::prelude::*; // flush() at least
use std::path::Path;
use std::rc::Rc;

extern crate jsparagus_ast as ast;
extern crate jsparagus_emitter as emitter;
extern crate jsparagus_interpreter as interpreter;
extern crate jsparagus_parser as parser;

use ast::arena;
use ast::dump::ASTDump;
use ast::source_atom_set::SourceAtomSet;
use ast::source_slice_list::SourceSliceList;
use ast::types::Program;
use bumpalo::Bump;
use interpreter::{create_global, evaluate, Object};
use parser::{is_partial_script, parse_script, ParseOptions};

use rustyline::error::ReadlineError;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Editor;
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

#[derive(Clone, Debug, Default)]
pub struct DemoStats {
    files_attempted: usize,
    files_parsed: usize,

    /// Total size of all the files attempted, in bytes.
    total_bytes: u64,
}

impl DemoStats {
    pub fn new() -> DemoStats {
        DemoStats::default()
    }

    pub fn new_single(size_bytes: u64, success: bool) -> DemoStats {
        DemoStats {
            files_attempted: 1,
            files_parsed: if success { 1 } else { 0 },
            total_bytes: size_bytes,
        }
    }

    pub fn add(&mut self, other: &DemoStats) {
        self.files_attempted += other.files_attempted;
        self.files_parsed += other.files_parsed;
        self.total_bytes += other.total_bytes;
    }
}

/// Try parsing a file.
///
/// Returns an Err only if opening or reading the file fails;
/// parse errors are simply printed to stdout.
fn parse_file(path: &Path, size_bytes: u64) -> io::Result<DemoStats> {
    print!("{}:", path.display());
    io::stdout().flush()?;
    let contents = match fs::read_to_string(path) {
        Err(err) => {
            println!(" error reading file: {}", err);
            return Ok(DemoStats::new_single(size_bytes, false));
        }
        Ok(s) => s,
    };
    let allocator = &Bump::new();
    let options = ParseOptions::new();
    let atoms = Rc::new(RefCell::new(SourceAtomSet::new()));
    let slices = Rc::new(RefCell::new(SourceSliceList::new()));
    let result = parse_script(allocator, &contents, &options, atoms, slices);
    let stats = DemoStats::new_single(size_bytes, result.is_ok());
    match result {
        Ok(_ast) => println!(" ok"),
        Err(err) => println!(" error: {}", err.message()),
    }
    Ok(stats)
}

/// Try parsing all the files in a directory, recursively.
///
/// Returns an Err only if reading a file or directory fails;
/// parse errors are simply printed to stdout.
fn parse_dir(path: &Path) -> io::Result<DemoStats> {
    let mut summary = DemoStats::new();
    for entry_result in fs::read_dir(&path)? {
        let entry = entry_result?;
        let file = entry.path();
        let metadata = entry.metadata()?;
        let stats = if metadata.is_file() {
            parse_file(&file, metadata.len())?
        } else if metadata.is_dir() {
            parse_dir(&file)?
        } else {
            DemoStats::new()
        };
        summary.add(&stats);
    }
    Ok(summary)
}

/// Try parsing a file, or all the files in a directory recursively.
///
/// Returns an Err only if reading a file or directory fails;
/// parse errors are simply printed to stdout.
pub fn parse_file_or_dir(filename: &impl AsRef<OsStr>) -> io::Result<DemoStats> {
    let path = Path::new(filename);
    let metadata = path.metadata()?;
    if metadata.is_dir() {
        parse_dir(path)
    } else {
        // No `if metadata.is_file()` here, we instead try opening it and let
        // that fail if this is some exotic filesystem thingy. That way the
        // user gets an error message.
        parse_file(Path::new(filename), metadata.len())
    }
}

/// Control level of stdio logging
pub struct Verbosity {
    /// Print AST
    pub ast: bool,

    /// Disassemble bytecode
    pub bytecode: bool,

    /// Debug print EmitResult
    pub emit_result: bool,
}

fn handle_script<'alloc>(
    verbosity: &Verbosity,
    program: &'alloc Program<'alloc>,
    atoms: SourceAtomSet<'alloc>,
    slices: SourceSliceList<'alloc>,
    global: Rc<RefCell<Object>>,
) {
    if verbosity.ast {
        program.dump_with_atoms(&mut io::stderr(), &atoms, &slices);
    }

    let options = emitter::EmitOptions::new();
    match emitter::emit(program, &options, atoms, slices) {
        Err(err) => {
            eprintln!("error: {}", err);
        }
        Ok(result) => {
            if verbosity.emit_result {
                println!("\n# EmitResult\n{:#?}", result);
            }

            if verbosity.bytecode {
                println!("\n# Bytecode\n{}", emitter::dis(&result.script.bytecode));
            }

            for (i, fun) in result.functions.iter().enumerate() {
                if verbosity.bytecode {
                    if fun.is_non_lazy() {
                        println!(
                            "\n# bytecode for functions[{}]\n{}",
                            i,
                            emitter::dis(&fun.non_lazy_script().bytecode)
                        );
                    }
                }
            }

            match evaluate(&result, global) {
                Err(err) => print!("error: {}", err),
                Ok(value) => println!("{:?}", value),
            }
        }
    }
}

#[derive(Completer, Helper, Highlighter, Hinter)]
struct InputValidator {}

impl Validator for InputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        let allocator = &Bump::new();
        let atoms = Rc::new(RefCell::new(SourceAtomSet::new()));
        let slices = Rc::new(RefCell::new(SourceSliceList::new()));
        match is_partial_script(allocator, ctx.input(), atoms, slices) {
            Ok(true) => Ok(ValidationResult::Incomplete),
            // We treat ParseErrors as "valid" so that they
            // can be handled by the REPL function.
            _ => Ok(ValidationResult::Valid(None)),
        }
    }
}

pub fn read_print_loop(verbosity: Verbosity) {
    let h = InputValidator {};
    let mut rl = Editor::new();
    rl.set_helper(Some(h));

    let global = create_global();

    loop {
        let input = rl.readline("> ");
        if let Err(err) = input {
            eprintln!("error: {:?}", err);
            break;
        }

        let input = input.unwrap();
        rl.add_history_entry(input.as_str());

        let allocator = &Bump::new();
        let atoms = Rc::new(RefCell::new(SourceAtomSet::new()));
        let slices = Rc::new(RefCell::new(SourceSliceList::new()));
        let script = parse_script(
            allocator,
            &input,
            &ParseOptions::new(),
            atoms.clone(),
            slices.clone(),
        );
        match script {
            Err(err) => {
                eprintln!("error: {}", err);
            }
            Ok(script) => {
                let program = arena::alloc(allocator, Program::Script(script.unbox()));

                handle_script(
                    &verbosity,
                    &program.unbox(),
                    atoms.replace(SourceAtomSet::new_uninitialized()),
                    slices.replace(SourceSliceList::new()),
                    global.clone(),
                );
            }
        }
    }
}
