use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Sastore;

impl Instruction for Sastore {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::sastore.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}