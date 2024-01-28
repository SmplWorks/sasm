use smpl_core_common::Instruction;
use crate::utils::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expr {
    Nop,
}

impl Expr {
    pub fn to_instructions(&self) -> Result<Vec<Instruction>> {
        use Expr::*;
        match self {
           Nop => Ok(vec![Instruction::nop()]) 
        }
    }
}
