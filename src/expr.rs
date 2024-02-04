use std::collections::HashMap;

use smpl_core_common::{Instruction, Register, Value};
use crate::utils::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Instruction(Instruction),
    DB(Vec<u8>),
    IdentifierDef(String),

    MovC2R(String, Register, bool),
}

impl Expr {
    pub fn to_instructions(&self, identifiers : &HashMap<String, u16>, offset : u16) -> Result<Vec<Instruction>> {
        match self {
            Expr::Instruction(instruction) => Ok(vec![*instruction]),
            Expr::DB(values) => Ok(values.iter().map(|value| Instruction::db(*value)).collect()),
            Expr::IdentifierDef(_) => Ok(vec![]),
            Self::MovC2R(ident, dest, relative) => Ok(vec![Instruction::movc2r(
                Value::word({
                    let ident_offset = *identifiers.get(ident).ok_or(Error::NoSuchIdentifier(ident.clone()))?;
                    if *relative {
                        if ident_offset < offset {
                            ident_offset.wrapping_sub(offset)
                        } else { 
                            offset.wrapping_sub(ident_offset)
                        }
                    } else { ident_offset }
                }),
                *dest
            )?]),
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u16 {
        match self {
            Expr::Instruction(instruction) => instruction.len(),
            Expr::DB(values) => values.len().try_into().unwrap(),
            Expr::IdentifierDef(_) => 0,
            Self::MovC2R(_, dest, _) =>
                Instruction::movc2r(Value::word(0), *dest).unwrap().len(), // TODO: Don't unwrap
        }
    }
}
