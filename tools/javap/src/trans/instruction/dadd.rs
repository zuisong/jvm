use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dadd;

impl Instruction for Dadd {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::dadd.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
