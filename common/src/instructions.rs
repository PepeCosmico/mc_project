use serde::{Deserialize, Serialize};

use crate::{message::Message, Error};

#[derive(Serialize, Deserialize, Debug, PartialEq, Message)]
pub enum Instruction {
    Difficulty(DifficultyLevel),
    SaveAll,
    Stop,
    Say(String),
    Whisper(String, String),
    Seed,
}

impl Instruction {
    pub fn as_command(&self) -> Vec<u8> {
        let mut string = match self {
            Self::Difficulty(level) => {
                let level_str: &str = level.into();
                String::from(format!("/difficulty {}", level_str))
            }
            Self::SaveAll => String::from("/save-all"),
            Self::Stop => String::from("/stop"),
            Self::Say(msg) => String::from(format!("/msg {}", msg)),
            Self::Whisper(player, msg) => String::from(format!("/w {} {}", player, msg)),
            Self::Seed => String::from("/seed"),
        };

        string.push_str("\n");
        string.as_bytes().to_vec()
    }
}

impl TryFrom<&Vec<&str>> for Instruction {
    type Error = Error;
    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            panic!("Trying to convert a empty Vec<&str> to a enum Instruction");
        }
        match value[0] {
            "difficulty" => Ok(Self::Difficulty(DifficultyLevel::Normal)), // TODO
            "save-all" => Ok(Self::SaveAll),
            "stop" => Ok(Self::Stop),
            "say" => Ok(Self::Say("Hello".to_string())), // TODO
            "w" => Ok(Self::Whisper("player".to_string(), "hello".to_string())), // TODO
            "seed" => Ok(Self::Seed),
            command => Err(Error::ParseInstructionError(command.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DifficultyLevel {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

impl From<&DifficultyLevel> for &str {
    fn from(value: &DifficultyLevel) -> Self {
        match value {
            DifficultyLevel::Peaceful => "peaceful",
            DifficultyLevel::Easy => "easy",
            DifficultyLevel::Normal => "normal",
            DifficultyLevel::Hard => "hard",
        }
    }
}

impl From<&str> for DifficultyLevel {
    fn from(value: &str) -> Self {
        match value {
            "peaceful" => Self::Peaceful,
            "easy" => Self::Easy,
            "normal" => Self::Normal,
            "hard" | &_ => Self::Hard,
        }
    }
}
