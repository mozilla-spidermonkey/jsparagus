mod pass;
mod scope;

use ast::*;

pub fn run(ast: &mut Program) {
    scope::postfix::pass(ast);
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::parse_script;
    use std::error::Error;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let parse_result = parse_script("wau")?;
        run(&mut ast::Program::Script(*parse_result));
        Ok(())
    }
}
