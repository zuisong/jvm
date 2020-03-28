use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Lushr;

impl Instruction for Lushr {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::lushr.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
