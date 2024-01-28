use smpl_core_common::Instruction;
use crate::utils::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Nop,
    DB(Vec<u8>),
}

impl Expr {
    pub fn to_instructions(&self) -> Result<Vec<Instruction>> {
        use Expr::*;
        match self {
           Nop => Ok(vec![Instruction::nop()]),
           DB(values) => Ok(values.iter().map(|value| Instruction::db(*value)).collect()),
        }
    }
}
