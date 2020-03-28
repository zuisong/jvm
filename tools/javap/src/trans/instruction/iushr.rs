use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Iushr;

impl Instruction for Iushr {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            name: OpCode::iushr.into(),
            code: codes[pc],
            icp: 0,
        };

        (info, pc + 1)
    }
}
