use crate::ast_emitter::AstEmitter;
use crate::bytecode_offset::{BytecodeOffset, BytecodeOffsetDiff};
use crate::emitter::EmitError;
use crate::emitter::InstructionWriter;
use crate::scope_notes::ScopeNoteIndex;
use ast::source_atom_set::SourceAtomSetIndex;

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
        let placeholder_offset = BytecodeOffsetDiff::uninitialized();
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
        emitter.emit.emit_jump_target_and_patch(&self.offsets);

        // If the previous opcode fall-through, it should have the same stack
        // depth.
        debug_assert!(emitter.emit.stack_depth() == self.depth);
    }

    pub fn patch_not_merge(self, emitter: &mut AstEmitter) {
        emitter.emit.emit_jump_target_and_patch(&self.offsets);
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

pub trait Breakable {
    fn register_break(&mut self, offset: BytecodeOffset);
    fn emit_break_target_and_patch(&mut self, emit: &mut InstructionWriter);
}

pub trait Continuable {
    fn register_continue(&mut self, offset: BytecodeOffset);

    fn emit_continue_target_and_patch(&mut self, emit: &mut InstructionWriter);
}

#[derive(Debug, PartialEq)]
pub struct LoopControl {
    scope_note_index: ScopeNoteIndex,
    breaks: Vec<BytecodeOffset>,
    continues: Vec<BytecodeOffset>,
    head: BytecodeOffset,
}

impl Breakable for LoopControl {
    fn register_break(&mut self, offset: BytecodeOffset) {
        // offset points to the location of the jump, which will need to be updated
        // once we emit the jump target in emit_jump_target_and_patch
        self.breaks.push(offset);
    }

    fn emit_break_target_and_patch(&mut self, emit: &mut InstructionWriter) {
        emit.emit_jump_target_and_patch(&self.breaks);
    }
}

impl Continuable for LoopControl {
    fn register_continue(&mut self, offset: BytecodeOffset) {
        // offset points to the location of the jump, which will need to be updated
        // once we emit the jump target in emit_jump_target_and_patch
        self.continues.push(offset);
    }

    fn emit_continue_target_and_patch(&mut self, emit: &mut InstructionWriter) {
        emit.emit_jump_target_and_patch(&self.continues);
    }
}

impl LoopControl {
    pub fn new(emit: &mut InstructionWriter, depth: u8, scope_note_index: ScopeNoteIndex) -> Self {
        let offset = LoopControl::open_loop(emit, depth);
        Self {
            scope_note_index,
            breaks: Vec::new(),
            continues: Vec::new(),
            head: offset,
        }
    }

    fn open_loop(emit: &mut InstructionWriter, depth: u8) -> BytecodeOffset {
        // Insert a Nop if needed to ensure the script does not start with a
        // JSOp::LoopHead. This avoids JIT issues with prologue code + try notes
        // or OSR. See bug 1602390 and bug 1602681.
        let mut offset = emit.bytecode_offset();
        if offset.offset == 0 {
            emit.nop();
            offset = emit.bytecode_offset();
        }
        // emit the jump target for the loop head
        emit.loop_head(depth);
        offset
    }

    pub fn close_loop(&mut self, emit: &mut InstructionWriter) {
        let offset = emit.bytecode_offset();
        let diff_to_head = self.head.diff_from(offset);

        emit.goto_(diff_to_head);
    }
}

#[derive(Debug, PartialEq)]
pub struct LabelControl {
    scope_note_index: ScopeNoteIndex,
    name: SourceAtomSetIndex,
    breaks: Vec<BytecodeOffset>,
    head: BytecodeOffset,
}

impl Breakable for LabelControl {
    fn register_break(&mut self, offset: BytecodeOffset) {
        // offset points to the location of the jump, which will need to be updated
        // once we emit the jump target in emit_jump_target_and_patch
        self.breaks.push(offset);
    }

    fn emit_break_target_and_patch(&mut self, emit: &mut InstructionWriter) {
        if !self.breaks.is_empty() {
            emit.emit_jump_target_and_patch(&self.breaks);
        }
    }
}

impl LabelControl {
    pub fn new(
        name: SourceAtomSetIndex,
        emit: &mut InstructionWriter,
        scope_note_index: ScopeNoteIndex,
    ) -> Self {
        let offset = emit.bytecode_offset();
        Self {
            scope_note_index,
            name,
            head: offset,
            breaks: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Control {
    Loop(LoopControl),
    Label(LabelControl),
}

impl Control {
    fn scope_note_index(&self) -> ScopeNoteIndex {
        match self {
            Control::Loop(control) => control.scope_note_index,
            Control::Label(control) => control.scope_note_index,
        }
    }
}

// Compared to C++ impl, this uses explicit stack struct,
// given Rust cannot store a reference of stack-allocated object into
// another object that has longer-lifetime.
pub struct ControlStructureStack {
    control_stack: Vec<Control>,
}

impl ControlStructureStack {
    pub fn new() -> Self {
        Self {
            control_stack: Vec::new(),
        }
    }

    pub fn open_loop(&mut self, emit: &mut InstructionWriter, scope_note_index: ScopeNoteIndex) {
        let depth = (self.control_stack.len() + 1) as u8;

        let new_loop = Control::Loop(LoopControl::new(emit, depth, scope_note_index));

        self.control_stack.push(new_loop);
    }

    pub fn open_label(
        &mut self,
        name: SourceAtomSetIndex,
        emit: &mut InstructionWriter,
        scope_note_index: ScopeNoteIndex,
    ) {
        let new_label = LabelControl::new(name, emit, scope_note_index);
        self.control_stack.push(Control::Label(new_label));
    }

    pub fn register_break(&mut self, offset: BytecodeOffset) {
        let innermost = self.innermost();

        match innermost {
            Control::Label(control) => control.register_break(offset),
            Control::Loop(control) => control.register_break(offset),
        }
    }

    pub fn register_continue(&mut self, offset: BytecodeOffset) {
        let innermost = self.innermost();
        match innermost {
            Control::Label(_) => panic!(
                "Should not register continue on a label. This should be caught by early errors."
            ),
            Control::Loop(control) => control.register_continue(offset),
        }
    }

    pub fn register_labelled_break(&mut self, label: SourceAtomSetIndex, offset: BytecodeOffset) {
        match self.find_labelled_control(label) {
            Control::Label(control) => control.register_break(offset),
            Control::Loop(control) => control.register_break(offset),
        }
    }

    pub fn register_labelled_continue(
        &mut self,
        label: SourceAtomSetIndex,
        offset: BytecodeOffset,
    ) {
        if let Some(control) = self.find_labelled_loop(label) {
            control.register_continue(offset);
        } else {
            panic!(
                "A labelled continue was passed, but no label was found. This should be caught by early errors"
            )
        }
    }

    pub fn find_labelled_loop(&mut self, label: SourceAtomSetIndex) -> Option<&mut LoopControl> {
        let label_index = self.find_labelled_index(label);
        // To find the associated loop for a label, we can take the label's index + 1, as the
        // associated loop should always be in the position after the label.
        let control = self.control_stack.get_mut(label_index + 1);
        match control {
            Some(Control::Loop(control)) => Some(control),
            _ => None,
        }
    }

    pub fn find_labelled_control(&mut self, label: SourceAtomSetIndex) -> &mut Control {
        self.control_stack
            .iter_mut()
            .find(|control| match control {
                Control::Label(control) => {
                    if control.name == label {
                        return true;
                    }
                    false
                }
                _ => false,
            })
            .expect("there should be a control with this label")
    }

    pub fn find_labelled_index(&mut self, label: SourceAtomSetIndex) -> usize {
        self.control_stack
            .iter()
            .position(|control| match control {
                Control::Label(control) => {
                    if control.name == label {
                        return true;
                    }
                    false
                }
                _ => false,
            })
            .expect("there should be a control with this label")
    }

    pub fn emit_continue_target_and_patch(&mut self, emit: &mut InstructionWriter) {
        let innermost = self.innermost();
        match innermost {
            Control::Label(_) => panic!(
                "Should not emit continue on a label. This should be caught by JS early errors"
            ),
            Control::Loop(control) => control.emit_continue_target_and_patch(emit),
        }
    }

    fn pop_control(&mut self) -> Control {
        self.control_stack
            .pop()
            .expect("There should be at least one control structure")
    }

    pub fn close_loop(&mut self, emit: &mut InstructionWriter) {
        let mut innermost = self.pop_control();
        match innermost {
            Control::Label(_) => panic!("Tried to close a loop, found a label."),
            Control::Loop(ref mut control) => {
                control.close_loop(emit);
                control.emit_break_target_and_patch(emit);
            }
        }
    }

    pub fn close_label(&mut self, emit: &mut InstructionWriter) {
        let mut innermost = self.pop_control();
        match innermost {
            Control::Label(ref mut control) => control.emit_break_target_and_patch(emit),
            Control::Loop(_) => panic!("Tried to close a label, found a loop."),
        }
    }

    pub fn innermost(&mut self) -> &mut Control {
        self.control_stack
            .last_mut()
            .expect("There should be at least one loop")
    }
}

struct RegisteredJump<F1>
where
    F1: Fn(&mut AstEmitter, BytecodeOffset),
{
    kind: JumpKind,
    register_offset: F1,
}

impl<F1> Jump for RegisteredJump<F1>
where
    F1: Fn(&mut AstEmitter, BytecodeOffset),
{
    fn jump_kind(&mut self) -> &JumpKind {
        &self.kind
    }
}

impl<F1> RegisteredJump<F1>
where
    F1: Fn(&mut AstEmitter, BytecodeOffset),
{
    pub fn emit(&mut self, emitter: &mut AstEmitter) {
        let offset = emitter.emit.bytecode_offset();
        self.emit_jump(emitter);
        (self.register_offset)(emitter, offset);
    }
}

// Struct for multiple jumps that point to the same target. Examples are breaks and loop conditions.
pub struct BreakEmitter {
    pub label: Option<SourceAtomSetIndex>,
}

impl BreakEmitter {
    pub fn emit(&mut self, emitter: &mut AstEmitter) {
        // step 1) create scope holes for the break until we are in the scope of the label or
        // the for loop of this statement
        let scope_note_holes = NonLocalExitControl {
            kind: NonLocalExitKind::Break,
        }
        .prepare_for_non_local_jump(emitter, self.label);

        // step 2) emit a registered jump on the appropriate nested control
        RegisteredJump {
            kind: JumpKind::Goto,
            register_offset: |emitter, offset| match self.label {
                Some(label) => emitter.control_stack.register_labelled_break(label, offset),
                None => emitter.control_stack.register_break(offset),
            },
        }
        .emit(emitter);

        // step 3) close each scope hole after the jump
        for hole in scope_note_holes.iter() {
            hole.emit_non_local_jump_end(&mut emitter.emit);
        }
    }
}

pub struct ContinueEmitter {
    pub label: Option<SourceAtomSetIndex>,
}

impl ContinueEmitter {
    pub fn emit(&mut self, emitter: &mut AstEmitter) {
        // step 1) create scope holes for the break until we are in the scope of the label or
        // the for loop of this statement
        let scope_note_holes = NonLocalExitControl {
            kind: NonLocalExitKind::Continue,
        }
        .prepare_for_non_local_jump(emitter, self.label);

        // step 2) emit a registered jump on the appropriate nested control
        RegisteredJump {
            kind: JumpKind::Goto,
            register_offset: |emitter, offset| match self.label {
                Some(label) => emitter
                    .control_stack
                    .register_labelled_continue(label, offset),
                None => emitter.control_stack.register_continue(offset),
            },
        }
        .emit(emitter);

        // step 3) close each scope hole after the jump
        for hole in scope_note_holes.iter() {
            hole.emit_non_local_jump_end(&mut emitter.emit);
        }
    }
}

pub struct WhileEmitter<F1, F2>
where
    F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub scope_note_index: ScopeNoteIndex,
    pub test: F1,
    pub block: F2,
}
impl<F1, F2> WhileEmitter<F1, F2>
where
    F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub fn emit(&mut self, emitter: &mut AstEmitter) -> Result<(), EmitError> {
        emitter
            .control_stack
            .open_loop(&mut emitter.emit, self.scope_note_index);

        (self.test)(emitter)?;

        // add a registered jump for the conditional statement
        RegisteredJump {
            kind: JumpKind::IfEq,
            register_offset: |emitter, offset| emitter.control_stack.register_break(offset),
        }
        .emit(emitter);

        (self.block)(emitter)?;

        emitter
            .control_stack
            .emit_continue_target_and_patch(&mut emitter.emit);

        // Merge point
        emitter.control_stack.close_loop(&mut emitter.emit);
        Ok(())
    }
}

pub struct DoWhileEmitter<F1, F2>
where
    F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub scope_note_index: ScopeNoteIndex,
    pub block: F2,
    pub test: F1,
}
impl<F1, F2> DoWhileEmitter<F1, F2>
where
    F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
    F2: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub fn emit(&mut self, emitter: &mut AstEmitter) -> Result<(), EmitError> {
        emitter
            .control_stack
            .open_loop(&mut emitter.emit, self.scope_note_index);

        (self.block)(emitter)?;

        emitter
            .control_stack
            .emit_continue_target_and_patch(&mut emitter.emit);

        (self.test)(emitter)?;

        // add a registered jump for the conditional statement
        RegisteredJump {
            kind: JumpKind::IfEq,
            register_offset: |emitter, offset| emitter.control_stack.register_break(offset),
        }
        .emit(emitter);

        // Merge point after cond fails
        emitter.control_stack.close_loop(&mut emitter.emit);
        Ok(())
    }
}

pub struct CForEmitter<'a, CondT, ExprT, InitFn, TestFn, UpdateFn, BlockFn>
where
    InitFn: Fn(&mut AstEmitter, &CondT) -> Result<(), EmitError>,
    TestFn: Fn(&mut AstEmitter, &ExprT) -> Result<(), EmitError>,
    UpdateFn: Fn(&mut AstEmitter, &ExprT) -> Result<(), EmitError>,
    BlockFn: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub scope_note_index: ScopeNoteIndex,
    pub maybe_init: &'a Option<CondT>,
    pub maybe_test: &'a Option<ExprT>,
    pub maybe_update: &'a Option<ExprT>,
    pub init: InitFn,
    pub test: TestFn,
    pub update: UpdateFn,
    pub block: BlockFn,
}
impl<'a, CondT, ExprT, InitFn, TestFn, UpdateFn, BlockFn>
    CForEmitter<'a, CondT, ExprT, InitFn, TestFn, UpdateFn, BlockFn>
where
    InitFn: Fn(&mut AstEmitter, &CondT) -> Result<(), EmitError>,
    TestFn: Fn(&mut AstEmitter, &ExprT) -> Result<(), EmitError>,
    UpdateFn: Fn(&mut AstEmitter, &ExprT) -> Result<(), EmitError>,
    BlockFn: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub fn emit(&mut self, emitter: &mut AstEmitter) -> Result<(), EmitError> {
        // Initialize the variable either by running an expression or assigning
        // ie) for(foo(); <test>; <update>) or for(var x = 0; <test>; <update)
        if let Some(init) = self.maybe_init {
            (self.init)(emitter, init)?;
        }

        // Emit loop head
        emitter
            .control_stack
            .open_loop(&mut emitter.emit, self.scope_note_index);

        // if there is a test condition (ie x < 3) emit it
        if let Some(test) = self.maybe_test {
            (self.test)(emitter, &test)?;

            // add a registered jump for the conditional statement
            RegisteredJump {
                kind: JumpKind::IfEq,
                register_offset: |emitter, offset| emitter.control_stack.register_break(offset),
            }
            .emit(emitter);
        }

        // emit the body of the for loop.
        (self.block)(emitter)?;

        // emit the target for any continues emitted in the body before evaluating
        // the update (if there is one) and continuing from the top of the loop.
        emitter
            .control_stack
            .emit_continue_target_and_patch(&mut emitter.emit);

        if let Some(update) = self.maybe_update {
            (self.update)(emitter, &update)?;
        }

        // Merge point after test fails (or there is a break statement)
        emitter.control_stack.close_loop(&mut emitter.emit);
        Ok(())
    }
}

pub struct LabelEmitter<F1>
where
    F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub scope_note_index: ScopeNoteIndex,
    pub name: SourceAtomSetIndex,
    pub body: F1,
}

impl<F1> LabelEmitter<F1>
where
    F1: Fn(&mut AstEmitter) -> Result<(), EmitError>,
{
    pub fn emit(&mut self, emitter: &mut AstEmitter) -> Result<(), EmitError> {
        emitter
            .control_stack
            .open_label(self.name, &mut emitter.emit, self.scope_note_index);

        (self.body)(emitter)?;

        emitter.control_stack.close_label(&mut emitter.emit);
        Ok(())
    }
}

pub enum NonLocalExitKind {
    Continue,
    Break,
}

pub struct NonLocalExitControl {
    pub kind: NonLocalExitKind,
}

impl NonLocalExitControl {
    pub fn prepare_for_non_local_jump(
        &mut self,
        emitter: &mut AstEmitter,
        label: Option<SourceAtomSetIndex>,
    ) -> Vec<ScopeNoteHoleEmitter> {
        let target_scope_note_index = match label {
            Some(label) => emitter
                .control_stack
                .find_labelled_control(label)
                .scope_note_index(),
            None => emitter.control_stack.innermost().scope_note_index(),
        };

        let mut holes = Vec::new();
        for maybe_scope_note_index in emitter
            .scope_stack
            .scope_indices_from(&target_scope_note_index)
            .iter()
            .rev()
        {
            if let Some(index) = maybe_scope_note_index {
                holes.push(ScopeNoteHoleEmitter {
                    scope_note_index: emitter.emit.enter_scope_hole(*index),
                });
            }
        }

        holes
    }
}

pub struct ScopeNoteHoleEmitter {
    scope_note_index: ScopeNoteIndex,
}

impl ScopeNoteHoleEmitter {
    pub fn emit_non_local_jump_end(&self, emit: &mut InstructionWriter) {
        emit.leave_scope_hole(self.scope_note_index);
    }
}
