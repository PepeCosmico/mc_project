use std::{io::Write, process::ChildStdin};

use common::instructions::Instruction;

use crate::error::{Error, Result};

pub fn process_instructions(
    msg: &impl Command,
    child_stdin: &mut Option<ChildStdin>,
) -> Result<()> {
    match child_stdin {
        Some(stdin) => {
            let bytes = msg.as_command()?;
            stdin.write_all(&bytes).unwrap();
        }
        None => (),
    }
    Ok(())
}

pub trait Command {
    fn as_command(&self) -> Result<Vec<u8>>;
}

impl Command for Instruction {
    fn as_command(&self) -> Result<Vec<u8>> {
        let mut string = match self {
            Self::Start => return Err(Error::CommandCreationError),
            Self::SaveAll => String::from("/save-all"),
            Self::Stop => String::from("/stop"),
            Self::Say(msg) => String::from(format!("/say {}", msg)),
            Self::Seed => String::from("/seed"),
        };
        string.push_str("\n");
        Ok(string.as_bytes().to_vec())
    }
}
