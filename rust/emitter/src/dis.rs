use crate::opcode::TABLE;
use std::fmt::Write;

pub fn dis(bc: &[u8]) -> String {
    let mut result = String::new();
    for &byte in bc {
        let mut found = false;
        for opcode in TABLE.iter() {
            if opcode.value == byte {
                writeln!(&mut result, "{}", opcode.name).unwrap();
                found = true;
                break;
            }
        }
        if !found {
            writeln!(&mut result, "{}", byte).unwrap();
        }
    }
    result
}
