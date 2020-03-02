use std::collections::HashMap;

/// Index into AtomSet.atoms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AtomSetIndex {
    index: usize,
}
impl AtomSetIndex {
    fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn into_raw(self) -> usize {
        self.index
    }
}

/// Set of atoms, including the following:
///
///   * atoms referred from bytecode
///   * variable names referred from scope data
///
/// WARNING: This set itself does *NOT* map to JSScript::atoms().
///          See atoms.rs.
#[derive(Debug)]
pub struct AtomSet<'alloc> {
    atoms: Vec<String>,

    /// Cache for the case the same string is inserted multiple times.
    atom_indices: HashMap<&'alloc str, AtomSetIndex>,
}

impl<'alloc> AtomSet<'alloc> {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            atom_indices: HashMap::new(),
        }
    }

    pub fn insert(&mut self, s: &'alloc str) -> AtomSetIndex {
        match self.atom_indices.get(s) {
            Some(index) => return *index,
            _ => {}
        }

        let index = self.atoms.len();
        self.atoms.push(s.to_string());
        let result = AtomSetIndex::new(index);
        self.atom_indices.insert(s, result);
        result
    }

    pub fn into_vec(self) -> Vec<String> {
        self.atoms
    }
}
