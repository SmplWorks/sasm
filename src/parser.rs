use smpl_core_common::Instruction;
use crate::{Expr, Token, Tokens, tokenize, utils::{Error, Result}};

fn parse_db(toks : &mut Tokens) -> Result<Expr> {
    let mut values = Vec::new();

    loop {
        let Some(t) = toks.next() else { return Err(Error::EOF("a number", "db")) };
        match t {
            Token::Number(value) =>
                values.push(value.try_into().map_err(|_| Error::NumberTooLarge(value, 8))?),

            _ => return Err(Error::UnexpectedToken(t, "db")),
        };

        let Some(t) = toks.next() else { break };
        match t {
            Token::Comma => (),
            _ => return Err(Error::UnexpectedToken(t, "db")),
        }
    }

    Ok(Expr::DB(values))
}

fn parse_toks(toks : &mut Tokens) -> Result<Option<Expr>> {
    let Some(t) = toks.next() else { return Ok(None) };

    use Token::*;
    Ok(Some(match t {
        Number(_) | Comma =>
            return Err(Error::UnexpectedToken(t, "parse_toks")),

        Nop => Expr::Nop,
        DB => parse_db(toks)?,
    }))
}

fn parse_line(code : &str) -> Result<Option<Expr>> {
    parse_toks(&mut tokenize(code))
}

fn parse_to_exprs(code : &str) -> Result<Vec<Expr>> {
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
    for expr in parse_to_exprs(code)? {
        res.append(&mut expr.to_instructions()?);
    }
    Ok(res)
}
