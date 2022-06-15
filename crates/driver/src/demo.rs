//! Functions to exercise the parser from the command line.

use std::cell::RefCell;
use std::convert::TryInto;
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
extern crate jsparagus_stencil as stencil;

use ast::arena;
use ast::dump::ASTDump;
use ast::source_atom_set::SourceAtomSet;
use ast::source_slice_list::SourceSliceList;
use ast::types::Program;
use bumpalo::Bump;
use interpreter::{create_global, evaluate, Object};
use parser::{is_partial_script, parse_script, ParseOptions};
use stencil::script::SourceExtent;

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
    extent: SourceExtent,
) {
    if verbosity.ast {
        program.dump_with_atoms(&mut io::stderr(), &atoms, &slices);
    }

    let options = emitter::EmitOptions::new(extent);
    match emitter::emit(program, &options, atoms, slices) {
        Err(err) => {
            eprintln!("error: {}", err);
        }
        Ok(result) => {
            if verbosity.emit_result {
                println!("\n# EmitResult\n{:#?}", result);
            }

            if verbosity.bytecode {
                for (i, script) in result.scripts.iter().enumerate() {
                    if !script.is_function() || script.is_non_lazy_function() {
                        let script_data_index: usize = script
                            .immutable_script_data
                            .expect("Non-lazy script should have ImmutableScriptData")
                            .into();
                        let script_data = &result.script_data_list[script_data_index];
                        println!(
                            "\n# bytecode for script[{}]\n{}",
                            i,
                            emitter::dis(&script_data.bytecode)
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

fn prompt(s: &'static str) -> bool {
    if let Err(err) = write!(std::io::stdout(), "{}", s) {
        eprintln!("error: {:?}", err);
        return false;
    }
    if let Err(err) = std::io::stdout().flush() {
        eprintln!("error: {:?}", err);
        return false;
    }

    true
}

fn read_line() -> Option<String> {
    if !prompt("js> ") {
        return None;
    }

    let mut input = String::new();
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Err(err) => {
                eprintln!("error: {:?}", err);
                return None;
            }
            Ok(0) => {
                return None;
            }
            _ => {}
        }
        input.push_str(line.as_str());

        let allocator = &Bump::new();
        let atoms = Rc::new(RefCell::new(SourceAtomSet::new()));
        let slices = Rc::new(RefCell::new(SourceSliceList::new()));
        match is_partial_script(allocator, input.as_str(), atoms, slices) {
            Ok(true) => {}
            Ok(false) => break,
            Err(err) => {
                eprintln!("error: {:?}", err);
                return None;
            }
        }

        if !prompt("... ") {
            return None;
        }
    }

    Some(input)
}

pub fn read_print_loop(verbosity: Verbosity) {
    let global = create_global();

    let mut line = 1;
    loop {
        let input = read_line();
        if input.is_none() {
            break;
        }

        let input = input.unwrap();
        let input_len = input.len();

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
                let extent = SourceExtent::top_level_script(input_len.try_into().unwrap(), line, 0);

                let program = arena::alloc(allocator, Program::Script(script.unbox()));
                handle_script(
                    &verbosity,
                    &program,
                    atoms.replace(SourceAtomSet::new_uninitialized()),
                    slices.replace(SourceSliceList::new()),
                    global.clone(),
                    extent,
                );
            }
        }

        line += 1;
    }
}
