use std::io;

use thiserror::Error as ThisError;

pub mod instructions;
pub mod message;
pub mod response;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Error parsing instructio")]
    ParseInstructionError,
    #[error("IO Error sending message")]
    SendMessageError(#[from] io::Error),
}
