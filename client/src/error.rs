use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Error parsing a instruction")]
    ParseInstructionError(#[from] common::Error),
}
