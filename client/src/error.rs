use std::io;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Error parsing a instruction")]
    ParseInstructionError(#[from] common::Error),
    #[error("Not a valid input")]
    InvalidInputError,
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Serializing Error")]
    SerializeError(#[from] bincode::Error),
}
