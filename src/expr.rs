use smpl_core_common::Instruction;
use crate::utils::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Instruction(Instruction),
    DB(Vec<u8>),
    IdentifierDef(String),
}

impl Expr {
    pub fn to_instructions(&self) -> Result<Vec<Instruction>> {
        match self {
            Expr::Instruction(instruction) => Ok(vec![*instruction]),
            Expr::DB(values) => Ok(values.iter().map(|value| Instruction::db(*value)).collect()),
            Expr::IdentifierDef(_) => Ok(vec![]),
        }
    }

    pub fn len(&self) -> u16 {
        match self {
            Expr::Instruction(instruction) => instruction.len(),
            Expr::DB(values) => values.len().try_into().unwrap(),
            Expr::IdentifierDef(_) => 0,
        }
    }
}
