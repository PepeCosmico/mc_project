use std::{io::Write, process::ChildStdin};

use common::{
    instructions::Instruction,
    message::{Message, MessageType},
};

use crate::{command::Command, error::Result};

pub fn process_instructions(msg: impl Message, child_stdin: &mut ChildStdin) -> Result<bool> {
    match msg.get_type() {
        MessageType::ServerCommand => {
            let instruction = msg.get_instruction();
            if instruction == &Instruction::Stop {
                return Ok(true);
            }
        }
        MessageType::MinecraftCommand => {
            let instruction = msg.get_instruction();
            let command = instruction.as_command()?;
            child_stdin.write_all(&command).unwrap();
        }
    }
    Ok(false)
}
