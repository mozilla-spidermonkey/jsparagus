use emitter::opcode::Opcode;
use emitter::EmitResult;

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

use crate::value::{to_boolean, to_number, JSValue};

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

trait Helpers {
    fn read_i32(&self, offset: usize) -> i32;
    fn read_offset(&self, offset: usize) -> isize;
}

impl Helpers for EmitResult {
    fn read_i32(&self, offset: usize) -> i32 {
        i32::from_le_bytes(self.bytecode[offset..offset + 4].try_into().unwrap())
    }

    fn read_offset(&self, offset: usize) -> isize {
        self.read_i32(offset) as isize
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
                stack.push(JSValue::Number(to_number(&lhs) + to_number(&rhs)))
            }

            Opcode::Sub => {
                let rhs = stack.pop().ok_or(EvalError::EmptyStack)?;
                let lhs = stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Number(to_number(&lhs) - to_number(&rhs)))
            }

            Opcode::Pos => {
                let v = stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Number(to_number(&v)));
            }

            Opcode::Neg => {
                let v = stack.pop().ok_or(EvalError::EmptyStack)?;
                stack.push(JSValue::Number(-to_number(&v)));
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

            Opcode::IfEq => {
                let b = to_boolean(&stack.pop().ok_or(EvalError::EmptyStack)?);
                if !b {
                    let offset = emit.read_offset(pc + 1);
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
            }

            Opcode::Goto => {
                let offset = emit.read_offset(pc + 1);
                pc = (pc as isize + offset) as usize;
                continue;
            }

            Opcode::And => {
                let cond = to_boolean(stack.last().ok_or(EvalError::EmptyStack)?);
                if !cond {
                    let offset = emit.read_offset(pc + 1);
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
            }

            Opcode::Or => {
                let cond = to_boolean(stack.last().ok_or(EvalError::EmptyStack)?);
                if cond {
                    let offset = emit.read_offset(pc + 1);
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
            }

            Opcode::Coalesce => {
                let last = stack.last().ok_or(EvalError::EmptyStack)?;
                match last {
                    JSValue::Null | JSValue::Undefined => {}
                    _ => {
                        let offset = emit.read_offset(pc + 1);
                        pc = (pc as isize + offset) as usize;
                        continue;
                    }
                }
            }

            Opcode::JumpTarget => {}

            Opcode::String => {
                let index = emit.read_i32(pc + 1) as usize;
                stack.push(JSValue::String(emit.strings[index].clone()))
            }

            Opcode::True => stack.push(JSValue::Boolean(true)),
            Opcode::False => stack.push(JSValue::Boolean(false)),
            Opcode::Undefined => stack.push(JSValue::Undefined),
            Opcode::Null => stack.push(JSValue::Null),

            _ => return Err(EvalError::NotImplemented(format!("{:?}", op))),
        }

        pc += op.instruction_length();
    }
}
