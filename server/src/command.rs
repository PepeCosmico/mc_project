use common::instructions::Instruction;

use crate::error::{Error, Result};

pub trait Command {
    fn as_command(&self) -> Result<Vec<u8>>;
}

impl Command for Instruction {
    fn as_command(&self) -> Result<Vec<u8>> {
        let mut string = match self {
            Self::Difficulty(level) => {
                let level_str: &str = level.into();
                String::from(format!("/difficulty {}", level_str))
            }
            Self::SaveAll => String::from("/save-all"),
            Self::Start => return Err(Error::CommandConversionError),
            Self::Stop => String::from("/stop"),
            Self::Say(msg) => String::from(format!("/msg {}", msg)),
            Self::Whisper(player, msg) => String::from(format!("/w {} {}", player, msg)),
            Self::Seed => String::from("/seed"),
        };

        string.push_str("\n");
        Ok(string.as_bytes().to_vec())
    }
}
