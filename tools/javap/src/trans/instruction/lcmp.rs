use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Lcmp;

impl Instruction for Lcmp {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::lcmp.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
