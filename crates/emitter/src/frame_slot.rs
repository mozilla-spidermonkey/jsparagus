/// Slot in the stack frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct FrameSlot {
    slot: u32,
}

impl FrameSlot {
    pub fn new(slot: u32) -> Self {
        Self { slot }
    }

    pub fn next(&mut self) {
        self.slot += 1;
    }

    pub fn into_raw(self) -> u32 {
        self.slot
    }
}
