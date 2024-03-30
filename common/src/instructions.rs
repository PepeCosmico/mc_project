use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Instruction {
    Difficulty(DifficultyLevel),
    SaveAll,
    Start,
    Stop,
    Say(String),
    Whisper(String, String),
    Seed,
}

impl Instruction {
    pub fn ser(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn deser(value: &Vec<u8>) -> Self {
        bincode::deserialize(&value).unwrap()
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
            "start" => Ok(Self::Start),
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
