use smpl_core_common::{Instruction, Value, Register};
use crate::{Expr, Token, Tokens, tokenize, utils::{Error, Result}};

fn parse_db_values(toks : &mut Tokens, ctx : &'static str) -> Result<Vec<i64>> {
    let mut values = Vec::new();

    loop {
        let Some(t) = toks.next() else { return Err(Error::EOF("a number", ctx)) };
        match t {
            Token::Number(value) => values.push(value),
            _ => return Err(Error::UnexpectedToken(t, ctx)),
        };

        let Some(t) = toks.next() else { break };
        match t {
            Token::Comma => (),
            _ => return Err(Error::UnexpectedToken(t, ctx)),
        }
    }

    Ok(values)
}

fn parse_db(toks : &mut Tokens) -> Result<Expr> {
    let mut values : Vec<u8> = Vec::new();
    for value in parse_db_values(toks, "db")? {
        values.push(value.try_into().map_err(|_| Error::NumberTooLarge(value, "byte"))?);
    }
    Ok(Expr::DB(values))
}

fn parse_dw(toks : &mut Tokens) -> Result<Expr> {
    let mut values : Vec<u16> = Vec::new();
    for value in parse_db_values(toks, "db")? {
        values.push(value.try_into().map_err(|_| Error::NumberTooLarge(value, "byte"))?);
    }
    Ok(Expr::DB(values.into_iter().flat_map(|value| [value as u8, (value >> 8) as u8]).collect()))
}

fn parse_comma(toks : &mut Tokens, ctx : &'static str) -> Result<(Token, Token)> {
    let Some(t1) = toks.next() else { return Err(Error::EOF("value", ctx)) };

    let Some(t2) = toks.next() else { return Err(Error::EOF("comma", ctx)) };
    if t2 != Token::Comma {
        return Err(Error::UnexpectedToken(t2, "mov"))
    }

    let Some(t3) = toks.next() else { return Err(Error::EOF("value", ctx)) };
    Ok((t1, t3))
}

fn parse_regs(toks : &mut Tokens, ctx : &'static str) -> Result<(Register, Register)> {
    let (t1, t2) = parse_comma(toks, "add")?;
    let Token::Register(r1) = t1 else { return Err(Error::UnexpectedToken(t1, ctx)) };
    let Token::Register(r2) = t2 else { return Err(Error::UnexpectedToken(t2, ctx)) };
    Ok((r1, r2))
}

fn parse_movc2r(value : i64, t2 : Token) -> Result<Expr> {
    match t2 {
        Token::Register(r2) => Ok(Expr::Instruction(Instruction::movc2r(Value::new(r2.width(), value as u16), r2)?)),

        _ => Err(Error::UnexpectedToken(t2, "mov")),
    }
}

fn parse_movr2x(r1 : Register, t2 : Token) -> Result<Expr> {
    match t2 {
        Token::Register(r2) => Ok(Expr::Instruction(Instruction::movr2r(r1, r2)?)),
        Token::Pointer(r2) => Ok(Expr::Instruction(Instruction::movr2m(r1, r2)?)),

        _ => Err(Error::UnexpectedToken(t2, "mov")),
    }
}

fn parse_movm2x(r1 : Register, t2 : Token) -> Result<Expr> {
    match t2 {
        Token::Register(r2) => Ok(Expr::Instruction(Instruction::movm2r(r1, r2)?)),

        _ => Err(Error::UnexpectedToken(t2, "mov")),
    }
}

fn parse_mov(toks : &mut Tokens) -> Result<Expr> {
    let (t1, t2) = parse_comma(toks, "mov")?;
    match t1 {
        Token::Number(value) => parse_movc2r(value, t2),
        Token::Register(r1) => parse_movr2x(r1, t2),
        Token::Pointer(r1) => parse_movm2x(r1, t2),

        _ => Err(Error::UnexpectedToken(t1, "mov")),
    }
}


fn parse_add(toks : &mut Tokens) -> Result<Expr> {
    let (r1, r2) = parse_regs(toks, "add")?;
    Ok(Expr::Instruction(Instruction::add(r1, r2)?))
}

fn parse_sub(toks : &mut Tokens) -> Result<Expr> {
    let (r1, r2) = parse_regs(toks, "add")?;
    Ok(Expr::Instruction(Instruction::sub(r1, r2)?))
}

fn parse_toks(toks : &mut Tokens) -> Result<Option<Expr>> {
    let Some(t) = toks.next() else { return Ok(None) };

    use Token::*;
    Ok(Some(match t {
        Register(_) | Pointer(_) | Number(_) | Comma =>
            return Err(Error::UnexpectedToken(t, "parse_toks")),

        Nop => Expr::Instruction(Instruction::Nop),
        DB => parse_db(toks)?,
        DW => parse_dw(toks)?,
        Mov => parse_mov(toks)?,
        Add => parse_add(toks)?,
        Sub => parse_sub(toks)?,
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
