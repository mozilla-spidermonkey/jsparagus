mod array_emitter;
mod ast_emitter;
mod compilation_info;
mod dis;
mod emitter;
mod emitter_scope;
mod forward_jump_emitter;
mod frame_slot;
mod free_name_tracker;
mod gcthings;
mod object_emitter;
pub mod opcode;
pub mod opcode_info;
mod reference_op_emitter;
mod scope;
mod scope_notes;
mod scope_pass;
mod script_atom_set;

extern crate jsparagus_ast as ast;

pub use crate::emitter::{EmitError, EmitOptions, EmitResult};
pub use crate::gcthings::GCThing;
pub use crate::scope::{BindingName, ScopeData};
pub use crate::scope_notes::ScopeNote;
pub use dis::dis;

use ast::source_atom_set::SourceAtomSet;

pub fn emit<'alloc>(
    ast: &mut ast::types::Program,
    options: &EmitOptions,
    atoms: SourceAtomSet<'alloc>,
) -> Result<EmitResult<'alloc>, EmitError> {
    let scope_data_map = scope_pass::generate_scope_data(ast);
    ast_emitter::emit_program(ast, options, atoms, scope_data_map)
}

#[cfg(test)]
mod tests {
    extern crate jsparagus_parser as parser;

    use super::{emit, EmitOptions};
    use crate::dis::*;
    use crate::opcode::*;
    use ast::source_atom_set::SourceAtomSet;
    use bumpalo::Bump;
    use parser::{parse_script, ParseOptions};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn bytecode(source: &str) -> Vec<u8> {
        let alloc = &Bump::new();
        let parse_options = ParseOptions::new();
        let atoms = Rc::new(RefCell::new(SourceAtomSet::new()));
        let parse_result =
            parse_script(alloc, source, &parse_options, atoms.clone()).expect("Failed to parse");
        // println!("{:?}", parse_result);

        let emit_options = EmitOptions::new();
        let bc = emit(
            &mut ast::types::Program::Script(parse_result.unbox()),
            &emit_options,
            atoms.replace(SourceAtomSet::new_uninitialized()),
        )
        .expect("Should work!")
        .bytecode;
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
                Opcode::GetGName as u8,
                0,
                0,
                0,
                0,
                Opcode::GImplicitThis as u8,
                0,
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

    #[test]
    fn literals() {
        assert_eq!(
            bytecode("true"),
            vec![
                Opcode::True as u8,
                Opcode::SetRval as u8,
                Opcode::RetRval as u8,
            ]
        );
        assert_eq!(
            bytecode("false"),
            vec![
                Opcode::False as u8,
                Opcode::SetRval as u8,
                Opcode::RetRval as u8,
            ]
        );
        //assert_eq!(
        //    bytecode("'hello world'"),
        //    vec![
        //        Opcode::String as u8, 0, 0, 0, 0,
        //        Opcode::SetRval as u8,
        //        Opcode::RetRval as u8,
        //    ]
        //);
    }
}
