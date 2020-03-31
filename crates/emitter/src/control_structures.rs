use super::emitter::BytecodeOffset;
use crate::ast_emitter::AstEmitter;
use crate::emitter::InstructionWriter;

// Control structures

#[derive(Debug)]
pub enum JumpKind {
    Coalesce,
    LogicalAnd,
    LogicalOr,
    IfEq,
    Goto,
}

trait Jump {
    fn jump_kind(&mut self) -> &JumpKind {
        &JumpKind::Goto
    }

    fn should_fallthrough(&mut self) -> bool {
        // a fallthrough occurs if the jump is a conditional jump and if the
        // condition doesn't met, the execution goes to the next opcode
        // instead of the target of the jump.
        match self.jump_kind() {
            JumpKind::Coalesce { .. }
            | JumpKind::LogicalOr { .. }
            | JumpKind::LogicalAnd { .. }
            | JumpKind::IfEq { .. } => true,

            JumpKind::Goto { .. } => false,
        }
    }

    fn emit_jump(&mut self, emitter: &mut AstEmitter) {
        // in the c++ bytecode emitter, the jumplist is emitted
        // and four bytes are used in order to save memory. We are not using that
        // here, so instead we are using a placeholder offset set to 0, which will
        // be updated later in patch_and_emit_jump_target.
        let placeholder_offset: i32 = 0;
        match self.jump_kind() {
            JumpKind::Coalesce { .. } => {
                emitter.emit.coalesce(placeholder_offset);
            }
            JumpKind::LogicalOr { .. } => {
                emitter.emit.or_(placeholder_offset);
            }
            JumpKind::LogicalAnd { .. } => {
                emitter.emit.and_(placeholder_offset);
            }
            JumpKind::IfEq { .. } => {
                emitter.emit.if_eq(placeholder_offset);
            }
            JumpKind::Goto { .. } => {
                emitter.emit.goto_(placeholder_offset);
            }
        }

        // The JITs rely on a jump target being emitted after the
        // conditional jump
        if self.should_fallthrough() {
            emitter.emit.jump_target();
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct JumpPatchEmitter {
    offsets: Vec<BytecodeOffset>,
    depth: usize,
}

impl JumpPatchEmitter {
    pub fn patch_merge(self, emitter: &mut AstEmitter) {
        emitter.emit.patch_and_emit_jump_target(self.offsets);

        // If the previous opcode fall-through, it should have the same stack
        // depth.
        debug_assert!(emitter.emit.stack_depth() == self.depth);
    }

    pub fn patch_not_merge(self, emitter: &mut AstEmitter) {
        emitter.emit.patch_and_emit_jump_target(self.offsets);
        // If the previous opcode doesn't fall-through, overwrite the stack
        // depth.
        emitter.emit.set_stack_depth(self.depth);
    }
}

// Struct for emitting bytecode for forward jump.
#[derive(Debug)]
pub struct ForwardJumpEmitter {
    pub jump: JumpKind,
}
impl Jump for ForwardJumpEmitter {
    fn jump_kind(&mut self) -> &JumpKind {
        &self.jump
    }
}

impl ForwardJumpEmitter {
    pub fn emit(&mut self, emitter: &mut AstEmitter) -> JumpPatchEmitter {
        let offsets = vec![emitter.emit.bytecode_offset()];
        self.emit_jump(emitter);
        let depth = emitter.emit.stack_depth();

        JumpPatchEmitter { offsets, depth }
    }
}

// Compared to C++ impl, this uses explicit stack struct,
// given Rust cannot store a reference of stack-allocated object into
// another object that has longer-lifetime.
pub struct LoopStack {
    loop_stack: Vec<LoopControl>,
}

impl LoopStack {
    pub fn new() -> Self {
        Self {
            loop_stack: Vec::new(),
        }
    }

    pub fn open_loop(&mut self, emit: &mut InstructionWriter) {
        let depth = (self.loop_stack.len() + 1) as u8;
        let new_loop = LoopControl::new(emit, depth);
        self.loop_stack.push(new_loop);
    }

    pub fn register_break(&mut self, offset: BytecodeOffset) {
        let innermost = self.innermost();
        innermost.register_break(offset);
    }

    pub fn close_loop(&mut self, emit: &mut InstructionWriter) {
        let innermost = self
            .loop_stack
            .pop()
            .expect("There should be at least one loop");
        innermost.emit_end_target(emit);
    }

    pub fn innermost(&mut self) -> &mut LoopControl {
        self.loop_stack
            .last_mut()
            .expect("There should be at least one loop")
    }
}

pub struct LoopControl {
    pub breaks: Vec<BytecodeOffset>,
    pub continues: Vec<BytecodeOffset>,
    head: BytecodeOffset,
}

impl LoopControl {
    pub fn new(emit: &mut InstructionWriter, depth: u8) -> Self {
        // Insert a Nop if needed to ensure the script does not start with a
        // JSOp::LoopHead. This avoids JIT issues with prologue code + try notes
        // or OSR. See bug 1602390 and bug 1602681.
        let mut offset = emit.bytecode_offset();
        if offset.offset == 0 {
            emit.nop();
            offset = emit.bytecode_offset();
        }
        emit.loop_head(depth);
        // emit the jump target for the loop head
        Self {
            breaks: Vec::new(),
            continues: Vec::new(),
            head: offset,
        }
    }

    pub fn register_break(&mut self, offset: BytecodeOffset) {
        // offset points to the location of the jump, which will need to be updated
        // once we emit the jump target in patch_and_emit_jump_target
        self.breaks.push(offset);
    }

    // TODO: fix continues so that they work with scopes correctly
    // pub fn emit_continue_target(&mut self, emit: &mut InstructionWriter) {
    //     if self.continues.len() > 0 {
    //         emit.patch_and_emit_jump_target((*self.continues).to_vec());
    //     }
    // }

    pub fn emit_end_target(self, emit: &mut InstructionWriter) {
        let offset = emit.bytecode_offset();
        emit.goto_(self.head.offset as i32);
        emit.patch_jump_to_target(self.head, offset);

        emit.patch_and_emit_jump_target(self.breaks);
    }
}

// Struct for multiple jumps that point to the same target. Examples are breaks and loop conditions.
pub struct BreakEmitter {
    pub jump: JumpKind,
}

impl Jump for BreakEmitter {
    fn jump_kind(&mut self) -> &JumpKind {
        &self.jump
    }
}

impl BreakEmitter {
    pub fn emit(&mut self, emitter: &mut AstEmitter) {
        // break { control, jumpkind }.emit(self)
        //      -> register break
        //      -> jump
        let offset = emitter.emit.bytecode_offset();
        emitter.loop_stack.register_break(offset);
        self.emit_jump(emitter);
    }
}
