use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ladd;

impl Instruction for Ladd {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::ladd.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
