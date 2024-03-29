use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Instructions {
    Difficulty(DifficultyLevel),
    SaveAll,
    Stop,
    Say(String),
    Whisper(String, String),
    Seed,
}

impl Instructions {
    pub fn as_command(&self) -> Vec<u8> {
        let mut string = match self {
            Self::Difficulty(level) => {
                let mut buf = String::from("/difficulty ");
                buf.push_str(level.as_str());
                buf
            }
            Self::SaveAll => String::from("/save-all"),
            _ => String::new(),
        };

        string.push_str("\n");
        string.as_bytes().to_vec()
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum DifficultyLevel {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

impl DifficultyLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Peaceful => "peaceful",
            Self::Easy => "easy",
            Self::Normal => "normal",
            Self::Hard => "hard",
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
