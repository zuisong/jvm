use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Lload_1;

impl Instruction for Lload_1 {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::lload_1.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
