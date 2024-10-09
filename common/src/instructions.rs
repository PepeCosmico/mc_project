use serde::{Deserialize, Serialize};

use crate::{message::Message, Error};

#[derive(Serialize, Deserialize, Debug, PartialEq, Message)]
pub enum Instruction {
    Help,
    Start,
    Status,
    SaveAll,
    Stop,
    Say(String),
    Seed,
    Op(String),
    Deop(String),
    WhitelistAdd(String),
}

impl TryFrom<&Vec<&str>> for Instruction {
    type Error = Error;
    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            panic!("Trying to convert a empty Vec<&str> to a enum Instruction");
        }
        match value[0] {
            "help" => Ok(Self::Help),
            "start" => Ok(Self::Start),
            "status" => Ok(Self::Status),
            "save-all" => Ok(Self::SaveAll),
            "stop" => Ok(Self::Stop),
            "say" => Ok(Self::Say(value[1].to_string())),
            "seed" => Ok(Self::Seed),
            "op" => Ok(Self::Op(value[1].to_string())),
            "deop" => Ok(Self::Deop(value[1].to_string())),
            "whitelist-add" => Ok(Self::WhitelistAdd(value[1].to_string())),
            _command => Err(Error::ParseInstructionError),
        }
    }
}
