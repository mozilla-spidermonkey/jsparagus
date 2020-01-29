use emitter::opcode::Opcode;
use emitter::EmitResult;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;

use crate::object::Object;
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
    fn read_u16(&self, offset: usize) -> u16;
    fn read_i32(&self, offset: usize) -> i32;
    fn read_u32(&self, offset: usize) -> u32;
    fn read_offset(&self, offset: usize) -> isize;
    fn read_atom(&self, offset: usize) -> String;
}

impl Helpers for EmitResult {
    fn read_u16(&self, offset: usize) -> u16 {
        u16::from_le_bytes(self.bytecode[offset..offset + 2].try_into().unwrap())
    }

    fn read_i32(&self, offset: usize) -> i32 {
        i32::from_le_bytes(self.bytecode[offset..offset + 4].try_into().unwrap())
    }

    fn read_u32(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.bytecode[offset..offset + 4].try_into().unwrap())
    }

    fn read_offset(&self, offset: usize) -> isize {
        self.read_i32(offset) as isize
    }

    fn read_atom(&self, offset: usize) -> String {
        self.strings[self.read_i32(offset) as usize].clone()
    }
}

fn print(_this_value: JSValue, args: &[JSValue]) -> JSValue {
    println!("{:?}", args);
    JSValue::Undefined
}

pub fn evaluate(emit: &EmitResult) -> Result<JSValue, EvalError> {
    let mut pc = 0;
    let mut stack = Vec::new();
    let mut rval = JSValue::Undefined;

    let global = Rc::new(RefCell::new(Object::new()));
    global
        .borrow_mut()
        .set("print".to_owned(), JSValue::NativeFunction(print));
    global
        .borrow_mut()
        .set("undefined".to_owned(), JSValue::Undefined);

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

            Opcode::BindGName => {
                // TODO: proper binding
                stack.push(JSValue::Object(global.clone()))
            }

            Opcode::GetGName => {
                let atom = emit.read_atom(pc + 1);
                stack.push(global.borrow().get(atom));
            }

            Opcode::SetGName => {
                let value = stack.pop().ok_or(EvalError::EmptyStack)?;
                let obj = stack.pop().ok_or(EvalError::EmptyStack)?;

                let atom = emit.read_atom(pc + 1);
                match obj {
                    JSValue::Object(ref obj) => {
                        obj.borrow_mut().set(atom, value.clone());
                    }
                    _ => return Err(EvalError::NotImplemented("not an object".to_owned())),
                }

                stack.push(value);
            }

            Opcode::InitElemArray => {
                let value = stack.pop().ok_or(EvalError::EmptyStack)?;
                let obj = stack.pop().ok_or(EvalError::EmptyStack)?;

                let index = emit.read_u32(pc + 1);
                match obj {
                    JSValue::Object(ref obj) => {
                        obj.borrow_mut().set(index.to_string(), value);
                    }
                    _ => return Err(EvalError::NotImplemented("not an object".to_owned())),
                }

                stack.push(obj);
            }

            Opcode::GImplicitThis => {
                // "The result is always `undefined` except when the name refers to a
                // binding in a non-syntactic `with` environment."
                stack.push(JSValue::Undefined);
            }

            Opcode::Call => {
                let argc = emit.read_u16(pc + 1) as usize;

                if stack.len() < argc {
                    return Err(EvalError::EmptyStack);
                }

                let args = stack.split_off(stack.len() - argc);
                let thisv = stack.pop().ok_or(EvalError::EmptyStack)?;
                let callee = stack.pop().ok_or(EvalError::EmptyStack)?;

                match callee {
                    JSValue::NativeFunction(fun) => {
                        stack.push(fun(thisv, &args));
                    }
                    _ => return Err(EvalError::NotImplemented("non function callee".to_owned())),
                }
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

            Opcode::NewArray => {
                stack.push(JSValue::Object(Rc::new(RefCell::new(Object::new()))));
            }

            Opcode::String => {
                stack.push(JSValue::String(emit.read_atom(pc + 1)));
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
