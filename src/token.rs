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
    Add,
    Sub,
    Jmp,
    AJmp,

    // Keywords
    Rel,
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
            "add" => ScannerAction::Return(Token::Add),
            "sub" => ScannerAction::Return(Token::Sub),
            "ajmp" => ScannerAction::Return(Token::AJmp),
            "jmp" => ScannerAction::Return(Token::Jmp),

            "rel" => ScannerAction::Return(Token::Rel),

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
