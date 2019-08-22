use crate::opcode::{Opcode, LENGTH};
use std::convert::TryFrom;
use std::fmt::Write;

/// Return a string form of the given bytecode.
pub fn dis(bc: &[u8]) -> String {
    let mut result = String::new();
    let mut iter = bc.iter();
    loop {
        let len = match iter.next() {
            Some(byte) => match Opcode::try_from(*byte) {
                Ok(op) => {
                    write!(&mut result, "{:?}", op).unwrap();
                    LENGTH[op as usize] - 1
                }
                Err(()) => {
                    write!(&mut result, "{}", byte).unwrap();
                    0
                }
            },
            None => break,
        };

        for _ in 0..len {
            write!(&mut result, " {}", iter.next().unwrap()).unwrap();
        }

        writeln!(&mut result).unwrap();
    }

    result
}
