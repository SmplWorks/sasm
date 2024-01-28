use smpl_core_common::Instruction;
use crate::utils::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Instruction(Instruction),
    DB(Vec<u8>),
}

impl Expr {
    pub fn to_instructions(&self) -> Result<Vec<Instruction>> {
        match self {
            Expr::Instruction(instruction) => Ok(vec![*instruction]),
            Expr::DB(values) => Ok(values.iter().map(|value| Instruction::db(*value)).collect()),
        }
    }
}
