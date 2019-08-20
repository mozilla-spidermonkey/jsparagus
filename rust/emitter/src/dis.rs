use std::fmt::Write;
use std::convert::TryFrom;
use crate::opcode::Opcode;

pub fn dis(bc: &[u8]) -> String {
    let mut result = String::new();
    for &byte in bc {
        let mut found = false;
        match Opcode::try_from(byte) {
            Ok(op) => {
                writeln!(&mut result, "{:?}", op).unwrap();
            }
            Err(()) => {
                writeln!(&mut result, "{}", byte).unwrap();
            }
        }
    }
    result
}
