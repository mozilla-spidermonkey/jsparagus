use ast::arena;
use ast::source_atom_set::SourceAtomSet;
use ast::source_slice_list::SourceSliceList;
use bumpalo::Bump;
use emitter::{emit, EmitOptions};
use parser::{parse_script, ParseOptions};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use stencil::script::SourceExtent;

use crate::{create_global, evaluate, EvalError, JSValue};

fn try_evaluate(source: &str) -> Result<JSValue, EvalError> {
    let alloc = &Bump::new();
    let parse_options = ParseOptions::new();
    let atoms = Rc::new(RefCell::new(SourceAtomSet::new()));
    let slices = Rc::new(RefCell::new(SourceSliceList::new()));
    let source_len = source.len();
    let parse_result = parse_script(alloc, source, &parse_options, atoms.clone(), slices.clone())
        .expect("Failed to parse");
    let extent = SourceExtent::top_level_script(source_len.try_into().unwrap(), 1, 0);
    let emit_options = EmitOptions::new(extent);
    let script = parse_result.unbox();
    let program = arena::alloc(alloc, ast::types::Program::Script(script));
    let result = emit(
        &program,
        &emit_options,
        atoms.replace(SourceAtomSet::new_uninitialized()),
        slices.replace(SourceSliceList::new()),
    )
    .expect("Should work!");
    evaluate(&result, create_global())
}

#[test]
fn test_numeric() {
    match try_evaluate("0") {
        Ok(JSValue::Number(n)) if n == 0.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("1") {
        Ok(JSValue::Number(n)) if n == 1.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("2") {
        Ok(JSValue::Number(n)) if n == 2.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("127") {
        Ok(JSValue::Number(n)) if n == 127.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("128") {
        Ok(JSValue::Number(n)) if n == 128.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("65535") {
        Ok(JSValue::Number(n)) if n == 65535.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("65536") {
        Ok(JSValue::Number(n)) if n == 65536.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("16777215") {
        Ok(JSValue::Number(n)) if n == 16777215.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("16777216") {
        Ok(JSValue::Number(n)) if n == 16777216.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("2147483647") {
        Ok(JSValue::Number(n)) if n == 2147483647.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("2147483648") {
        Ok(JSValue::Number(n)) if n == 2147483648.0 => (),
        _ => panic!("wrong result"),
    }
}

#[test]
fn test_arithmetic() {
    match try_evaluate("1 + 1") {
        Ok(JSValue::Number(n)) if n == 2.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("3 - 2") {
        Ok(JSValue::Number(n)) if n == 1.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("-2") {
        Ok(JSValue::Number(n)) if n == -2.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("+2") {
        Ok(JSValue::Number(n)) if n == 2.0 => (),
        _ => panic!("wrong result"),
    }
}

#[test]
fn test_ternary_conditional() {
    match try_evaluate("true ? 1 : 37") {
        Ok(JSValue::Number(n)) if n == 1.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("false ? 1 : 37") {
        Ok(JSValue::Number(n)) if n == 37.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("1 ? true : false") {
        Ok(JSValue::Boolean(true)) => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("0 ? true : false") {
        Ok(JSValue::Boolean(false)) => (),
        _ => panic!("wrong result"),
    }
}

#[test]
fn test_gname() {
    match try_evaluate("undefined") {
        Ok(JSValue::Undefined) => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("a = true; a") {
        Ok(JSValue::Boolean(true)) => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("a = true; a = false; a") {
        Ok(JSValue::Boolean(false)) => (),
        _ => panic!("wrong result"),
    }
}

#[test]
fn test_if() {
    match try_evaluate("if (true) 1; else 2;") {
        Ok(JSValue::Number(n)) if n == 1.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("if (false) 1; else 2;") {
        Ok(JSValue::Number(n)) if n == 2.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("if (true) 1;") {
        Ok(JSValue::Number(n)) if n == 1.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("if (false) 1;") {
        Ok(JSValue::Undefined) => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("var x; if (true) { x = 1; } else { x = 2; } x") {
        Ok(JSValue::Number(n)) if n == 1.0 => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("var x; if (false) { x = 1; } else { x = 2; } x") {
        Ok(JSValue::Number(n)) if n == 2.0 => (),
        _ => panic!("wrong result"),
    }
}

#[test]
fn test_call() {
    match try_evaluate("print(1)") {
        Ok(JSValue::Undefined) => (),
        _ => panic!("wrong result"),
    }

    match try_evaluate("Math.sqrt(36)") {
        Ok(JSValue::Number(n)) if n == 6.0 => (),
        _ => panic!("wrong result"),
    }
}

#[test]
fn test_file() {
    match try_evaluate(include_str!("tests.js")) {
        Ok(JSValue::Undefined) => (),
        _ => panic!("rval of tests.js should be 'undefined'"),
    }
}
