use thiserror::Error as ThisError;

pub mod instructions;
pub mod message;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Error parsing instructio")]
    ParseInstructionError,
}
