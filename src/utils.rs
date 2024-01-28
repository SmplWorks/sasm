use crate::Token;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("reached end of file while expecting {0} for {1}")]
    EOF(&'static str, &'static str),

    #[error("unexpected token {0:?} for {1}")]
    UnexpectedToken(Token, &'static str),

    #[error("number too large to fit in a {1} ({0})")]
    NumberTooLarge(i64, &'static str),

    #[error("{0}")]
    CoreCommon(smpl_core_common::utils::Error),

    #[error("{0}")]
    External(String),
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<smpl_core_common::utils::Error> for Error {
		fn from(value: smpl_core_common::utils::Error) -> Self {
				Self::CoreCommon(value)
		}
}
