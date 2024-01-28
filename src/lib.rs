mod parser;
pub use parser::parse;

mod expr;
pub use expr::Expr;

pub mod utils;

#[cfg(test)]
mod test;

pub fn compile(code : &str) -> utils::Result<Vec<u8>> {
    Ok(parse(code)?.into_iter().flat_map(|inst| inst.compile()).collect())
}
