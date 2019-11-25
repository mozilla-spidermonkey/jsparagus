use bumpalo;

use parser::ParseError;

fn main() {
    println!("Warning: The disassembler we're using here is extremely inaccurate.");
    loop {
        let alloc = &bumpalo::Bump::new();
        match parser::read_script_interactively(alloc, "js> ", "..> ") {
            Err(ParseError::UnexpectedEnd) => {
                println!();
                break;
            }
            Err(err) => {
                eprintln!("error: {}", err);
            }
            Ok(script) => {
                let mut program = ast::types::Program::Script(script.unbox());
                match emitter::emit(&mut program) {
                    Err(err) => {
                        eprintln!("error: {}", err);
                    }
                    Ok(emit_result) => {
                        println!("{}", emitter::dis(&emit_result.bytecode));
                    }
                }
            }
        }
    }
}
