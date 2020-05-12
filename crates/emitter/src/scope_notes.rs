use crate::bytecode_offset::BytecodeOffset;
use crate::gcthings::GCThingIndex;

/// Maps to js::ScopeNote in m-c/js/src/vm//JSScript.h.
#[derive(Debug)]
pub struct ScopeNote {
    pub index: GCThingIndex,
    pub start: BytecodeOffset,
    pub end: BytecodeOffset,
    pub parent: Option<ScopeNoteIndex>,
}

impl ScopeNote {
    fn new(index: GCThingIndex, start: BytecodeOffset, parent: Option<ScopeNoteIndex>) -> Self {
        Self {
            index,
            start: start.clone(),
            end: start,
            parent,
        }
    }
}

/// Index into ScopeNoteList.notes.
#[derive(Debug, Clone, Copy)]
pub struct ScopeNoteIndex {
    index: usize,
}

impl ScopeNoteIndex {
    fn new(index: usize) -> Self {
        Self { index }
    }
}

impl From<ScopeNoteIndex> for usize {
    fn from(index: ScopeNoteIndex) -> usize {
        index.index
    }
}

/// List of scope notes.
///
/// Maps to JSScript::scopeNotes() in js/src/vm/JSScript.h.
pub struct ScopeNoteList {
    notes: Vec<ScopeNote>,
}

impl ScopeNoteList {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn enter_scope(
        &mut self,
        index: GCThingIndex,
        offset: BytecodeOffset,
        parent: Option<ScopeNoteIndex>,
    ) -> ScopeNoteIndex {
        let note_index = self.notes.len();
        self.notes.push(ScopeNote::new(index, offset, parent));
        ScopeNoteIndex::new(note_index)
    }

    fn get_scope_hole_gcthing_index(&self, scope_note_index: &ScopeNoteIndex) -> GCThingIndex {
        self.notes
            .get(scope_note_index.index)
            .expect("Scope note should exist")
            .index
    }

    pub fn enter_scope_hole(
        &mut self,
        maybe_enclosing_scope_note_index: &Option<ScopeNoteIndex>,
        body_index: GCThingIndex,
        offset: BytecodeOffset,
    ) -> ScopeNoteIndex {
        let parent_index = ScopeNoteIndex::new(self.notes.len() - 1);
        let gcthing_index = match maybe_enclosing_scope_note_index {
            Some(index) => self.get_scope_hole_gcthing_index(index),
            None => body_index,
        };
        self.enter_scope(gcthing_index, offset, Some(parent_index))
    }

    pub fn leave_scope(&mut self, index: ScopeNoteIndex, offset: BytecodeOffset) {
        self.notes[usize::from(index)].end = offset;
    }
}

impl From<ScopeNoteList> for Vec<ScopeNote> {
    fn from(list: ScopeNoteList) -> Vec<ScopeNote> {
        list.notes
    }
}
