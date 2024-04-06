use serde::{Deserialize, Serialize};

use crate::{message::Message, Error};

#[derive(Serialize, Deserialize, Debug, PartialEq, Message)]
pub enum Instruction {
    Start,
    Status,
    SaveAll,
    Stop,
    Say(String),
    Seed,
}

impl TryFrom<&Vec<&str>> for Instruction {
    type Error = Error;
    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            panic!("Trying to convert a empty Vec<&str> to a enum Instruction");
        }
        match value[0] {
            "start" => Ok(Self::Start),
            "status" => Ok(Self::Status),
            "save-all" => Ok(Self::SaveAll),
            "stop" => Ok(Self::Stop),
            "say" => Ok(Self::Say("Hello".to_string())), // TODO
            "seed" => Ok(Self::Seed),
            _command => Err(Error::ParseInstructionError),
        }
    }
}
