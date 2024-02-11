#[allow(unused_imports)] // Clippy is glitching!
use std::str::FromStr;

use smpl_core_common::Register;
use smpl_parser::{Scanner, ScannerAction, Token as PToken};
use crate::utils::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Misc
    Comment(String),
    IdentifierDef(String),
    IdentifierRef(String),
    Register(Register),
    Pointer(Register),
    Number(i64),
    Comma,

    // Instructions
    Nop,
    DB,
    DW,

    Mov,
    Push,
    Pop,

    Add,
    Sub,
    Not,
    And,
    Or,
    Shl,
    Shr,
    Shre,
    Cmp,

    AJmp,
    Jmp,
    Jeq,
    Jneq,
    Jlt,
    Jgt,
    Jleq,
    Jgeq,
    Jo,
    Jno,
    Call,
    Ret,

    Int,
    Sti,
    Cli,
}

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }
}

pub type Tokens = Scanner<Token>;

fn convert_tokens(toks : Vec<PToken>) -> Result<Vec<Token>> {
    let mut scanner = Scanner::new(toks.into());
    scanner.collect(|toks| match toks {
        [PToken::Comment(s)] => ScannerAction::Return(Token::Comment(s.to_string())),
        [PToken::Number(x)] => ScannerAction::Return(Token::Number(*x)),

        [PToken::Ident(op)] => match &**op {
            "nop" => ScannerAction::Return(Token::Nop),
            "db" => ScannerAction::Return(Token::DB),
            "dw" => ScannerAction::Return(Token::DW),

            "mov" => ScannerAction::Return(Token::Mov),
            "push" => ScannerAction::Return(Token::Push),
            "pop" => ScannerAction::Return(Token::Pop),

            "add" => ScannerAction::Return(Token::Add),
            "sub" => ScannerAction::Return(Token::Sub),
            "not" => ScannerAction::Return(Token::Not),
            "and" => ScannerAction::Return(Token::And),
            "or" => ScannerAction::Return(Token::Or),
            "shl" => ScannerAction::Return(Token::Shl),
            "shr" => ScannerAction::Return(Token::Shr),
            "shre" => ScannerAction::Return(Token::Shre),
            "cmp" => ScannerAction::Return(Token::Cmp),

            "ajmp" => ScannerAction::Return(Token::AJmp),
            "jmp" => ScannerAction::Return(Token::Jmp),
            "jeq" => ScannerAction::Return(Token::Jeq),
            "jneq" => ScannerAction::Return(Token::Jneq),
            "jlt" => ScannerAction::Return(Token::Jlt),
            "jgt" => ScannerAction::Return(Token::Jgt),
            "jleq" => ScannerAction::Return(Token::Jleq),
            "jgeq" => ScannerAction::Return(Token::Jgeq),
            "jo" => ScannerAction::Return(Token::Jo),
            "jno" => ScannerAction::Return(Token::Jno),
            "call" => ScannerAction::Return(Token::Call),
            "ret" => ScannerAction::Return(Token::Ret),

            "int" => ScannerAction::Return(Token::Int),
            "sti" => ScannerAction::Return(Token::Sti),
            "cli" => ScannerAction::Return(Token::Cli),

            _ if Register::from_str(op).is_ok()
                => ScannerAction::Return(Token::Register(Register::from_str(op).unwrap())),
            _ => ScannerAction::Request(Token::IdentifierRef(op.to_owned())),
        }
        [PToken::Ident(op), PToken::Punct(':')] => ScannerAction::Return(Token::IdentifierDef(op.to_owned())),

        [PToken::Punct('[')]
            => ScannerAction::Require,
        [PToken::Punct('['), PToken::Ident(reg)]
            if Register::from_str(reg).is_ok()
            => ScannerAction::Require,
        [PToken::Punct('['), PToken::Ident(reg), PToken::Punct(']')]
            if Register::from_str(reg).is_ok()
            => ScannerAction::Return(Token::Pointer(Register::from_str(reg).unwrap())),

        [PToken::Punct(',')] => ScannerAction::Return(Token::Comma),

        _ => ScannerAction::None,
    }).map_err(|err| Error::External(err.to_string()))
}

pub fn tokenize(code : &str) -> Result<Scanner<Token>> {
    let ptoks = smpl_parser::tokenize(code);
    let toks = convert_tokens(ptoks)?.into_iter().filter(|t| !t.is_comment()).collect();
    Ok(Scanner::new(toks))
}
