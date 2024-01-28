use smpl_core_common::Instruction;
use crate::{Expr, utils::Result};

pub fn parse_line(_code : &str) -> Result<Option<Expr>> {
    Ok(None)
}

pub fn parse_exprs(code : &str) -> Result<Vec<Expr>> {
    let mut res = Vec::new();
    for line in code.lines() {
        if let Some(expr) = parse_line(line)? {
            res.push(expr);
        }
    }
    Ok(res)
}

pub fn parse(code : &str) -> Result<Vec<Instruction>> {
    let mut res = Vec::new();
    for expr in parse_exprs(code)? {
        res.push(expr.to_instruction()?);
    }
    Ok(res)
}
