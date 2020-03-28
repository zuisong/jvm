use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Lstore;

impl Instruction for Lstore {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::lstore.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 2)
    }
}
