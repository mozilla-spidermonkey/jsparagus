use super::emitter::{AtomIndex, EmitError};
use crate::ast_emitter::AstEmitter;

#[derive(Debug, PartialEq)]
enum SetOpReferenceKind {
    GlobalName(AtomIndex),
    #[allow(dead_code)]
    Prop(AtomIndex),
    #[allow(dead_code)]
    Elem,
}

// See SetOpReferenceEmitter.
// Use struct to hide the details from the consumer.
#[derive(Debug)]
#[must_use]
pub struct SetOpReference {
    kind: SetOpReferenceKind,
}
impl SetOpReference {
    fn new(kind: SetOpReferenceKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, PartialEq)]
enum CallOpKind {
    Normal,
    // FIXME: Support eval, call, apply etc.
}

// See CallOpReferenceEmitter.
// Use struct to hide the details from the consumer.
#[derive(Debug)]
#[must_use]
pub struct CallOpReference {
    kind: CallOpKind,
}
impl CallOpReference {
    fn new(kind: CallOpKind) -> Self {
        Self { kind }
    }
}

// Struct for emitting bytecode for get operation.
//
// Usage:
//
//   `name;`
//      GetEmitter::emit_name(self, name);
//
//   `obj.key;`
//      GetEmitter::emit_prop(self, |emitter| {
//          emitter.emit_expression(obj)
//      }, key)?;
//
//   `obj[key];`
//      GetEmitter::emit_elem(self, |emitter| {
//          emitter.emit_expression(obj)
//      }, |emitter| {
//          emitter.emit_expression(key)
//      })?;
//
pub struct GetEmitter {}
impl GetEmitter {
    pub fn emit_name(emitter: &mut AstEmitter, name: &str) {
        let name_index = emitter.emit.get_atom_index(name);

        //              [stack]

        // FIXME: Support non-global case.
        emitter.emit.get_g_name(name_index);
        //              [stack] VAL
    }

    pub fn emit_prop<F>(
        emitter: &mut AstEmitter,
        obj_emitter: F,
        key: &str,
    ) -> Result<(), EmitError>
    where
        F: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        let key_index = emitter.emit.get_atom_index(key);

        //              [stack]

        obj_emitter(emitter)?;
        //              [stack] OBJ

        emitter.emit.get_prop(key_index);
        //              [stack] VAL

        Ok(())
    }

    pub fn emit_prop_super<F>(
        emitter: &mut AstEmitter,
        this_emitter: F,
        key: &str,
    ) -> Result<(), EmitError>
    where
        F: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        let key_index = emitter.emit.get_atom_index(key);

        //              [stack]

        this_emitter(emitter)?;
        //              [stack] THIS

        emitter.emit.callee();
        //              [stack] THIS CALLEE

        emitter.emit.super_base();
        //              [stack] THIS OBJ

        emitter.emit.get_prop_super(key_index);
        //              [stack] VAL

        Ok(())
    }

    pub fn emit_elem<F1, F2>(
        emitter: &mut AstEmitter,
        obj_emitter: F1,
        key_emitter: F2,
    ) -> Result<(), EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        //              [stack]

        obj_emitter(emitter)?;
        //              [stack] OBJ

        key_emitter(emitter)?;
        //              [stack] OBJ KEY

        emitter.emit.get_elem();
        //              [stack] VAL

        Ok(())
    }

    pub fn emit_elem_super<F1, F2>(
        emitter: &mut AstEmitter,
        this_emitter: F1,
        key_emitter: F2,
    ) -> Result<(), EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        //              [stack]

        this_emitter(emitter)?;
        //              [stack] THIS

        key_emitter(emitter)?;
        //              [stack] THIS KEY

        emitter.emit.callee();
        //              [stack] THIS KEY CALLEE

        emitter.emit.super_base();
        //              [stack] THIS KEY OBJ

        emitter.emit.get_elem_super();
        //              [stack] VAL

        Ok(())
    }
}

// See CallEmitter.
pub struct CallOpReferenceEmitter {}
impl CallOpReferenceEmitter {
    pub fn emit_name(emitter: &mut AstEmitter, name: &str) -> CallOpReference {
        let name_index = emitter.emit.get_atom_index(name);

        //              [stack]

        // FIXME: Support non-global case.
        emitter.emit.get_g_name(name_index);
        //              [stack] CALLEE

        // FIXME: Support non-global cases.
        emitter.emit.g_implicit_this(name_index);
        //              [stack] CALLEE THIS

        CallOpReference::new(CallOpKind::Normal)
    }

    pub fn emit_prop<F>(
        emitter: &mut AstEmitter,
        obj_emitter: F,
        key: &str,
    ) -> Result<CallOpReference, EmitError>
    where
        F: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        let key_index = emitter.emit.get_atom_index(key);

        //              [stack]

        obj_emitter(emitter)?;
        //              [stack] THIS

        emitter.emit.dup();
        //              [stack] THIS THIS

        // FIXME: Support super.
        emitter.emit.call_prop(key_index);
        //              [stack] THIS CALLEE

        emitter.emit.swap();
        //              [stack] CALLEE THIS

        Ok(CallOpReference::new(CallOpKind::Normal))
    }

    #[allow(dead_code)]
    pub fn emit_elem<F1, F2>(
        emitter: &mut AstEmitter,
        obj_emitter: F1,
        key_emitter: F2,
    ) -> Result<CallOpReference, EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        //              [stack]

        obj_emitter(emitter)?;
        //              [stack] THIS

        emitter.emit.dup();
        //              [stack] THIS THIS

        key_emitter(emitter)?;
        //              [stack] THIS THIS KEY

        // FIXME: Support super.
        emitter.emit.call_elem();
        //              [stack] THIS CALLEE

        emitter.emit.swap();
        //              [stack] CALLEE THIS

        Ok(CallOpReference::new(CallOpKind::Normal))
    }
}

// Struct for emitting bytecode for call operation.
//
// Usage:
//
//   `name(a);`
//      CallEmitter::emit(self, |emitter| {
//          Ok(CallOpReferenceEmitter::emit_name(emitter, name))
//      }, |emitter| {
//          // Return the number of arguments.
//          emitter.emit_expression(a)?;
//          Ok(1)
//      })?;
//
//   `obj.key();`
//      CallEmitter::emit(self, |emitter| {
//          CallOpReferenceEmitter::emit_prop(emitter, |emitter| {
//              emitter.emit_expression(obj)
//          }, key)
//      }, |emitter| {
//          Ok(0)
//      })?;
//
//   `obj[key](a, b, c);`
//      CallEmitter::emit(self, |emitter| {
//          CallOpReferenceEmitter::emit_elem(emitter, |emitter| {
//              emitter.emit_expression(obj)
//          }, |emitter| {
//              emitter.emit_expression(key)
//          })
//      }, |emitter| {
//          emitter.emit_expression(a)?;
//          emitter.emit_expression(b)?;
//          emitter.emit_expression(c)?;
//          Ok(3)
//      })?;
//
pub struct CallEmitter {}
impl CallEmitter {
    pub fn emit<F1, F2>(
        emitter: &mut AstEmitter,
        callee_emitter: F1,
        arguments_emitter: F2,
    ) -> Result<(), EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<CallOpReference, EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<usize, EmitError>,
    {
        //              [stack]

        let reference = callee_emitter(emitter)?;
        //              [stack] CALLEE THIS

        // FIXME: Support spread.
        let len = arguments_emitter(emitter)?;
        //              [stack] CALLEE THIS ARGS...

        match reference.kind {
            CallOpKind::Normal => {
                emitter.emit.call(len as u16);
                //      [stack] VAL
            }
        }

        Ok(())
    }
}

// Struct for emitting bytecode for new operation.
//
// Usage:
//
//   `new expr(a);`
//      NewEmitter::emit(self, |emitter| {
//          emitter.emit_expression(expr)
//      }, |emitter| {
//          // Return the number of arguments.
//          emitter.emit_expression(a)?;
//          Ok(1)
//      })?;
//
pub struct NewEmitter {}
impl NewEmitter {
    pub fn emit<F1, F2>(
        emitter: &mut AstEmitter,
        callee_emitter: F1,
        arguments_emitter: F2,
    ) -> Result<(), EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<usize, EmitError>,
    {
        //              [stack]

        callee_emitter(emitter)?;
        //              [stack] CALLEE

        emitter.emit.is_constructing();
        //              [stack] CALLEE JS_IS_CONSTRUCTING

        // FIXME: Support spread.
        let len = arguments_emitter(emitter)?;
        //              [stack] CALLEE JS_IS_CONSTRUCTING ARGS...

        emitter.emit.dup_at(len as u32 + 1);
        //              [stack] CALLEE JS_IS_CONSTRUCTING ARGS... CALLEE

        emitter.emit.new_(len as u16);
        //              [stack] VAL

        Ok(())
    }
}

// See AssignmentEmitter.
pub struct SetOpReferenceEmitter {}
impl SetOpReferenceEmitter {
    pub fn emit_name(emitter: &mut AstEmitter, name: &str) -> SetOpReference {
        let name_index = emitter.emit.get_atom_index(name);

        //              [stack]

        // FIXME: Support non-global case.
        emitter.emit.bind_g_name(name_index);
        //              [stack] GLOBAL

        SetOpReference::new(SetOpReferenceKind::GlobalName(name_index))
    }

    #[allow(dead_code)]
    pub fn emit_prop<F>(
        emitter: &mut AstEmitter,
        obj_emitter: F,
        key: &str,
    ) -> Result<SetOpReference, EmitError>
    where
        F: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        let key_index = emitter.emit.get_atom_index(key);

        //              [stack]

        obj_emitter(emitter)?;
        //              [stack] OBJ

        Ok(SetOpReference::new(SetOpReferenceKind::Prop(key_index)))
    }

    #[allow(dead_code)]
    pub fn emit_elem<F1, F2>(
        emitter: &mut AstEmitter,
        obj_emitter: F1,
        key_emitter: F2,
    ) -> Result<SetOpReference, EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        //              [stack]

        obj_emitter(emitter)?;
        //              [stack] OBJ

        key_emitter(emitter)?;
        //              [stack] OBJ KEY

        Ok(SetOpReference::new(SetOpReferenceKind::Elem))
    }
}

// Struct for emitting bytecode for assignment operation.
//
// Usage:
//
//   `name = expr;`
//      AssignmentEmitter::emit(self, |emitter| {
//          Ok(SetOpReferenceEmitter::emit_name(emitter, name))
//      }, |emitter| {
//          emitter.emit_expression(expr)
//      })?;
//
//   `obj.key = expr;`
//      AssignmentEmitter::emit(self, |emitter| {
//          SetOpReferenceEmitter::emit_prop(emitter, |emitter| {
//              emitter.emit_expression(obj)
//          }, key)
//      }, |emitter| {
//          emitter.emit_expression(expr)
//      })?;
//
//   `obj[key] = expr;`
//      AssignmentEmitter::emit(self, |emitter| {
//          SetOpReferenceEmitter::emit_elem(emitter, |emitter| {
//              emitter.emit_expression(obj)
//          }, |emitter| {
//            emitter.emit_expression(key)
//          })
//      }, |emitter| {
//          emitter.emit_expression(expr)
//      })?;
//
pub struct AssignmentEmitter {}
impl AssignmentEmitter {
    pub fn emit<F1, F2>(
        emitter: &mut AstEmitter,
        lhs_emitter: F1,
        rhs_emitter: F2,
    ) -> Result<(), EmitError>
    where
        F1: Fn(&mut AstEmitter) -> Result<SetOpReference, EmitError>,
        F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    {
        let reference = lhs_emitter(emitter)?;

        rhs_emitter(emitter)?;

        match reference.kind {
            SetOpReferenceKind::GlobalName(name_index) => {
                //      [stack] GLOBAL

                // FIXME: Support non-global cases.
                emitter.emit.set_g_name(name_index);
                //      [stack] VAL
            }
            SetOpReferenceKind::Prop(key_index) => {
                //      [stack] OBJ

                // FIXME: Support strict mode and super.
                emitter.emit.set_prop(key_index);
                //      [stack] VAL
            }
            SetOpReferenceKind::Elem => {
                //      [stack] OBJ KEY

                // FIXME: Support strict mode and super.
                emitter.emit.set_elem();
                //      [stack] VAL
            }
        }

        Ok(())
    }

    // FIXME: Support compound assignment
}

// FIXME: Add increment
