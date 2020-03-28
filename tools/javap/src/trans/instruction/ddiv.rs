use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ddiv;

impl Instruction for Ddiv {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::ddiv.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
