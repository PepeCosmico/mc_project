use thiserror::Error as ThisError;

pub mod instructions;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Error parsing instructio")]
    ParseInstructionError(String),
}
