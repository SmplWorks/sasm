use crate::Token;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("reached end of file while expecting {0} for {1}")]
    EOF(&'static str, &'static str),

    #[error("unexpected token {0:?} for {1}")]
    UnexpectedToken(Token, &'static str),

    #[error("number too large to fit in {1}-bits ({0})")]
    NumberTooLarge(i64, u8),
}
pub type Result<T> = std::result::Result<T, Error>;
