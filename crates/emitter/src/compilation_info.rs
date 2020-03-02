use crate::atomset::AtomSet;

pub struct CompilationInfo<'alloc> {
    pub atoms: AtomSet<'alloc>,
}

impl<'alloc> CompilationInfo<'alloc> {
    pub fn new(atoms: AtomSet<'alloc>) -> Self {
        Self { atoms }
    }
}
