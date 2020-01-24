use emitter::opcode::Opcode;
use emitter::EmitResult;

use std::convert::TryFrom;
use std::fmt;

use crate::value::{to_number, JSValue};

/// The error of evaluating JS bytecode.
#[derive(Clone, Debug)]
pub enum EvalError {
    NotImplemented(String),
    EmptyStack,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::NotImplemented(message) => write!(f, "not implemented: {}", message),
            EvalError::EmptyStack => write!(f, "trying to pop from empty stack"),
        }
    }
}

pub fn evaluate(emit: &EmitResult) -> Result<JSValue, EvalError> {
    let mut pc = 0;
    let mut stack = Vec::new();
    let mut rval = JSValue::Undefined;

    loop {
        let op = match Opcode::try_from(emit.bytecode[pc]) {
            Ok(op) => op,
            Err(_) => {
                return Err(EvalError::NotImplemented(format!(
                    "{} is not an opcode",
                    emit.bytecode[pc]
                )))
            }
        };

        match op {
            Opcode::Int8 => stack.push(JSValue::Number(emit.bytecode[pc + 1] as f64)),

            Opcode::Add => {
                let rhs = stack.pop().ok_or(EvalError::EmptyStack)?;
                let lhs = stack.pop().ok_or(EvalError::EmptyStack)?;
                // TODO: Add is special, i.e. string concat
                stack.push(JSValue::Number(to_number(lhs) + to_number(rhs)))
            }

            Opcode::Sub => {
                let rhs = stack.pop().ok_or(EvalError::EmptyStack)?;
                let lhs = stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Number(to_number(lhs) - to_number(rhs)))
            }

            Opcode::Pos => {
                let v = stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Number(to_number(v)));
            }

            Opcode::Neg => {
                let v = stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Number(-to_number(v)));
            }

            Opcode::Void => {
                stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Undefined);
            }

            Opcode::Pop => {
                stack.pop().ok_or(EvalError::EmptyStack)?;
            }

            Opcode::SetRval => {
                rval = stack.pop().ok_or(EvalError::EmptyStack)?;
            }

            Opcode::RetRval => {
                return Ok(rval);
            }

            Opcode::Undefined => stack.push(JSValue::Undefined),
            Opcode::Null => stack.push(JSValue::Null),

            _ => return Err(EvalError::NotImplemented(format!("{:?}", op))),
        }

        pc += op.instruction_length();
    }
}
