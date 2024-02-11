use std::collections::HashMap;

use smpl_core_common::{Instruction, Value, Register};
use crate::{Expr, Token, Tokens, tokenize, utils::{Error, Result}};

fn parse_db_values(toks : &mut Tokens, ctx : &'static str) -> Result<Vec<i64>> {
    let mut values = Vec::new();

    loop {
        let Some(t) = toks.pop() else { return Err(Error::EOF("a number", ctx)) };
        match t {
            Token::Number(value) => values.push(value),
            _ => return Err(Error::UnexpectedToken(t, ctx)),
        };

        let Some(t) = toks.pop() else { break };
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
    let Some(t1) = toks.pop() else { return Err(Error::EOF("value", ctx)) };

    let Some(t2) = toks.pop() else { return Err(Error::EOF("comma", ctx)) };
    if t2 != Token::Comma {
        return Err(Error::UnexpectedToken(t2, "mov"))
    }

    let Some(t3) = toks.pop() else { return Err(Error::EOF("value", ctx)) };
    Ok((t1, t3))
}

fn parse_zero(op : Token, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Nop => Instruction::Nop,
        Ret => Instruction::Ret,
        Cli => Instruction::Cli,

        _ => return Err(Error::UnexpectedToken(op, "parse_zero")),
    }))
}

fn parse_one_r(op : Token, reg : Register, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Push => Instruction::push(reg),
        Pop => Instruction::pop(reg),

        Not => Instruction::not(reg),

        AJmp => Instruction::ajmp(reg),
        Jmp => Instruction::jmp(reg),
        Jeq => Instruction::jeq(reg),
        Jneq => Instruction::jneq(reg),
        Jlt => Instruction::jlt(reg),
        Jgt => Instruction::jgt(reg),
        Jleq => Instruction::jleq(reg),
        Jgeq => Instruction::jgeq(reg),
        Jo => Instruction::jo(reg),
        Jno => Instruction::jno(reg),
        Call => Instruction::callr(reg),

        Int => Instruction::int(reg),
        Sti => Instruction::sti(reg),

        _ => return Err(Error::UnexpectedToken(op, "parse_one_r")),
    }?))
}

fn parse_one_c(op : Token, value : i64, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Call => Instruction::callc(Value::from(value as u16)),

        _ => return Err(Error::UnexpectedToken(op, "parse_one_c")),
    }?))
}

fn parse_one(op : Token, toks : &mut Tokens) -> Result<Expr> {
    let Some(t) = toks.pop() else { return Err(Error::EOF("value", "parse_one")) };

    match t {
        Token::Register(reg) => parse_one_r(op, reg, toks),
        Token::Number(value) => parse_one_c(op, value, toks),

        _ => Err(Error::UnexpectedToken(t, "parse_one")),
    }
}

fn parse_two_r2r(op : Token, r1 : Register, r2 : Register, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Mov => Instruction::movr2r(r1, r2),

        Add => Instruction::addr2r(r1, r2),
        Sub => Instruction::subr2r(r1, r2),
        And => Instruction::andr2r(r1, r2),
        Or => Instruction::orr2r(r1, r2),
        Cmp => Instruction::cmpr2r(r1, r2),

        _ => return Err(Error::UnexpectedToken(op, "parse_two_r2r")),
    }?))
}

fn parse_two_r2p(op : Token, r1 : Register, r2 : Register, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Mov => Instruction::movr2m(r1, r2),

        _ => return Err(Error::UnexpectedToken(op, "parse_two_r2p")),
    }?))
}

fn parse_two_r(op : Token, r1 : Register, t2 : Token, toks : &mut Tokens) -> Result<Expr> {
    match t2 {
        Token::Register(r2) => parse_two_r2r(op, r1, r2, toks),
        Token::Pointer(r2) => parse_two_r2p(op, r1, r2, toks),

        _ => Err(Error::UnexpectedToken(t2, "parse_two_r")),
    }
}

fn parse_two_c2r(op : Token, value : Value, reg : Register, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Mov => Instruction::movc2r(value, reg),

        Add => Instruction::addc2r(value, reg),
        Sub => Instruction::subc2r(value, reg),
        And => Instruction::andc2r(value, reg),
        Or => Instruction::orc2r(value, reg),
        Cmp => Instruction::cmpc2r(value, reg),
        Shl => Instruction::shl(value, reg),
        Shr => Instruction::shr(value, reg),
        Shre => Instruction::shre(value, reg),

        _ => return Err(Error::UnexpectedToken(op, "parse_two_c2r")),
    }?))
}

fn parse_two_c(op : Token, v1 : i64, t2 : Token, toks : &mut Tokens) -> Result<Expr> {
    match t2 {
        Token::Register(reg) => parse_two_c2r(op, Value::new(reg.width(), v1 as u16), reg, toks),

        _ => Err(Error::UnexpectedToken(t2, "parse_two_c")),
    }
}

fn parse_two_p2r(op : Token, r1 : Register, r2 : Register, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(Expr::Instruction(match op {
        Mov => Instruction::movm2r(r1, r2)?,

        _ => return Err(Error::UnexpectedToken(op, "parse_two_p2r")),
    }))
}

fn parse_two_p(op : Token, r1 : Register, t2 : Token, toks : &mut Tokens) -> Result<Expr> {
    match t2 {
        Token::Register(r2) => parse_two_p2r(op, r1, r2, toks),

        _ => Err(Error::UnexpectedToken(t2, "parse_two_p")),
    }
}

fn parse_two_l2r(op : Token, label : String, reg : Register, _toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    Ok(match op {
        Mov => Expr::MovC2R(label, reg, false),

        _ => return Err(Error::UnexpectedToken(op, "parse_two_p2r")),
    })
}

fn parse_two_l(op : Token, l1 : String, t2 : Token, toks : &mut Tokens) -> Result<Expr> {
    match t2 {
        Token::Register(r2) => parse_two_l2r(op, l1, r2, toks),

        _ => Err(Error::UnexpectedToken(t2, "parse_two_l")),
    }
}

fn parse_two(op : Token, toks : &mut Tokens) -> Result<Expr> {
    let (t1, t2) = parse_comma(toks, "parse_two")?;

    match t1 {
        Token::Register(r1) => parse_two_r(op, r1, t2, toks),
        Token::Pointer(r1) => parse_two_p(op, r1, t2, toks),
        Token::Number(v1) => parse_two_c(op, v1, t2, toks),
        Token::IdentifierRef(label) => parse_two_l(op, label, t2, toks),

        _ => Err(Error::UnexpectedToken(t1, "parse_two")),
    }
}

fn parse_toks(t : Token, toks : &mut Tokens) -> Result<Expr> {
    use Token::*;
    match t {
        IdentifierDef(ident) => Ok(Expr::IdentifierDef(ident)),
        DB => parse_db(toks),
        DW => parse_dw(toks),

        Nop | Ret | Cli
            => parse_zero(t, toks),

        Push | Pop |
        Not |
        AJmp | Jmp | Jeq | Jneq | Jlt | Jgt | Jleq | Jgeq | Jo | Jno | Call |
        Int | Sti
            => parse_one(t, toks),

        Mov |
        Add | Sub | And | Or | Shl | Shr | Shre | Cmp
            => parse_two(t, toks),

        _ => Err(Error::UnexpectedToken(t, "parse_toks")),
    }
}

fn parse_to_exprs(code : &str) -> Result<Vec<Expr>> {
    let mut res = Vec::new();

    let mut toks = tokenize(code)?;
    while let Some(t) = toks.pop() {
        res.push(parse_toks(t, &mut toks)?)
    }

    Ok(res)
}

pub fn parse(code : &str) -> Result<(Vec<Instruction>, HashMap<String, u16>)> {
    let exprs = parse_to_exprs(code)?;
    let mut identifiers = HashMap::new();
    let mut offset = 0;
    for expr in exprs.iter() {
        if let Expr::IdentifierDef(ident) = expr {
            identifiers.insert(ident.clone(), offset);
        };

        offset += expr.len();
    }
    
    let mut res = Vec::new();
    let mut offset = 0;
    for expr in exprs.iter() {
        res.append(&mut expr.to_instructions(&identifiers, offset)?);

        offset += expr.len();
    }
    Ok((res, identifiers))
}
