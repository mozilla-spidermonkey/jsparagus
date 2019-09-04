fn main() {}
/*
fn main() {
    println!("Warning: The disassembler we're using here is extremely inaccurate.");
    loop {
        match parser::read_script_interactively("js> ", "..> ") {
            Err(err) => {
                if err.is::<parser::UserExit>() {
                    println!();
                    break;
                }
                eprintln!("error: {}", err);
            }
            Ok(script) => {
                let mut program = ast::Program::Script(*script);
                let result = emitter::emit(&mut program);
                println!("{}", emitter::dis(&result.bytecode));
            }
        }
    }
}
*/
