use thiserror::Error as ThisError;

use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Common crate error")]
    CommonError(#[from] common::Error),
    #[error("IO Error")]
    IOError(#[from] io::Error),
    #[error("Serializing Error")]
    SerializeError(#[from] bincode::Error),
    #[error("Read response timeout ended")]
    ReadResponseTimeoutError,
}
