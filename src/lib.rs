mod parser;
pub use parser::parse;

mod token;
pub use token::{Token, Tokens, tokenize};

mod expr;
pub use expr::Expr;

pub mod utils;

#[cfg(test)]
mod test;

pub fn compile(code : &str) -> utils::Result<Vec<u8>> {
    let (instructions, _) = parse(code)?;
    Ok(instructions.into_iter().flat_map(|inst| inst.compile()).collect())
}
