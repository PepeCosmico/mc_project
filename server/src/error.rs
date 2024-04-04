use std::io;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Could not spawn the child process for the mc server")]
    IOError(#[from] io::Error),
    #[error("Command creation error")]
    CommandCreationError,
}
