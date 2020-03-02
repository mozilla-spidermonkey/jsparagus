use crate::atomset::AtomSetIndex;
use indexmap::set::IndexSet;

/// Index into AtomList.atoms.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AtomIndex {
    index: u32,
}
impl AtomIndex {
    fn new(index: u32) -> Self {
        Self { index }
    }

    pub fn into_raw(self) -> u32 {
        self.index
    }
}

/// List of atoms referred from bytecode.
///
/// Maps to JSScript::atoms() in js/src/vm/JSScript.h.
pub struct AtomList {
    atoms: IndexSet<AtomSetIndex>,
}

impl AtomList {
    pub fn new() -> Self {
        Self {
            atoms: IndexSet::new(),
        }
    }

    pub fn insert(&mut self, value: AtomSetIndex) -> AtomIndex {
        let (index, _) = self.atoms.insert_full(value);
        AtomIndex::new(index as u32)
    }

    pub fn into_vec(mut self) -> Vec<AtomSetIndex> {
        self.atoms.drain(..).collect()
    }
}
