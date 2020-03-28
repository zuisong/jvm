use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ifge;

impl Instruction for Ifge {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::ifge.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 3)
    }
}
