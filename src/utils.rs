#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
}
pub type Result<T> = std::result::Result<T, Error>;
