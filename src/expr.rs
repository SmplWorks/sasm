use smpl_core_common::Instruction;
use crate::utils::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expr {
}

impl Expr {
    pub fn to_instruction(&self) -> Result<Instruction> {
        todo!()
    }
}
