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
#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn get_scope_hole_gcthing_index(&self, reference_scope_index: ScopeNoteIndex) -> GCThingIndex {
        if reference_scope_index.index >= 1 {
            return self
                .notes
                .get(reference_scope_index.index - 1)
                .unwrap()
                .index;
        }
        GCThingIndex::new(0)
    }

    pub fn enter_scope_hole(
        &mut self,
        reference_scope_index: ScopeNoteIndex,
        offset: BytecodeOffset,
    ) -> ScopeNoteIndex {
        let note_index = self.notes.len();
        let parent_index = ScopeNoteIndex::new(note_index - 1);
        let reference_gcthing_index = self.get_scope_hole_gcthing_index(reference_scope_index);
        self.enter_scope(reference_gcthing_index, offset, Some(parent_index))
    }

    pub fn leave_scope(&mut self, index: ScopeNoteIndex, offset: BytecodeOffset) {
        self.notes[usize::from(index)].end = offset;
    }

    pub fn current_index(&self) -> ScopeNoteIndex {
        let note_index = self.notes.len();
        ScopeNoteIndex::new(note_index)
    }
}

impl From<ScopeNoteList> for Vec<ScopeNote> {
    fn from(list: ScopeNoteList) -> Vec<ScopeNote> {
        list.notes
    }
}
