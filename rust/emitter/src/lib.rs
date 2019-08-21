mod ast_emitter;
mod dis;
mod emitter;
mod lower;
mod opcode;

pub use emitter::EmitResult;

pub fn emit(ast: &mut ast::Program) -> EmitResult {
    lower::run(ast);
    ast_emitter::emit_program(ast)
}

#[cfg(test)]
mod tests {
    use super::emit;
    use crate::dis::*;
    use crate::opcode::*;
    use parser::parse_script;

    fn bytecode(source: &str) -> Vec<u8> {
        let parse_result = parse_script(source).expect("Failed to parse");
        // println!("{:?}", parse_result);
        let bc = emit(&mut ast::Program::Script(*parse_result)).bytecode;
        println!("{}", dis(&bc));
        bc
    }

    #[test]
    fn it_works() {
        assert_eq!(
            bytecode("2 + 2"),
            vec![
                Opcode::Int8 as u8,
                2,
                Opcode::Int8 as u8,
                2,
                Opcode::Add as u8,
                Opcode::SetRval as u8,
                Opcode::RetRval as u8,
            ]
        )
    }

    #[test]
    fn dis_call() {
        assert_eq!(
            bytecode("dis()"),
            vec![
                Opcode::GetGname as u8,
                0,
                0,
                0,
                0,
                Opcode::GImplicitThis as u8,
                1,
                0,
                0,
                0,
                Opcode::Call as u8,
                0,
                0,
                Opcode::SetRval as u8,
                Opcode::RetRval as u8,
            ]
        )
    }

    // #[test]
    // fn let_return() {
    //     assert_eq!(
    //         bytecode("let x = 2; return x;"),
    //         vec![
    //             UNINITIALIZED.value,
    //             INITLEXICAL.value,
    //             0,
    //             0,
    //             0,
    //             POP.value,
    //             INT8.value,
    //             2,
    //             INITLEXICAL.value,
    //             0,
    //             0,
    //             0,
    //             POP.value,
    //             GETLOCAL.value,
    //             0,
    //             0,
    //             0,
    //             SETRVAL.value,
    //             DEBUGLEAVELEXICALENV.value,
    //             RETRVAL.value,
    //             DEBUGLEAVELEXICALENV.value,
    //             RETRVAL.value,
    //         ]
    //     )
    // }
}
