use bumpalo::Bump;
use emitter::{emit, EmitOptions};
use parser::{parse_script, ParseOptions};

use crate::{evaluate, EvalError, JSValue};

fn try_evaluate(source: &str) -> Result<JSValue, EvalError> {
    let alloc = &Bump::new();
    let parse_options = ParseOptions::new();
    let parse_result = parse_script(alloc, source, &parse_options).expect("Failed to parse");
    let emit_options = EmitOptions::new();
    let emit_result = emit(
        &mut ast::types::Program::Script(parse_result.unbox()),
        &emit_options,
    )
    .expect("Should work!");
    evaluate(&emit_result)
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
}

#[test]
fn test_call() {
    match try_evaluate("print(1)") {
        Ok(JSValue::Undefined) => (),
        _ => panic!("wrong result"),
    }
}
