use common::instructions::Instruction;

use crate::{
    error::{Error, Result},
    mc_server::McServer,
};

pub trait Command {
    fn as_command(&self) -> Result<Vec<u8>>;
    fn proccess_command(&self, mc_server: &mut McServer) -> Result<()>;
}

impl Command for Instruction {
    fn as_command(&self) -> Result<Vec<u8>> {
        let mut string = match self {
            Self::Start => return Err(Error::CommandCreationError),
            Self::Status => return Err(Error::CommandCreationError),
            Self::SaveAll => String::from("/save-all"),
            Self::Stop => String::from("/stop"),
            Self::Say(msg) => String::from(format!("/say {}", msg)),
            Self::Seed => String::from("/seed"),
        };
        string.push_str("\n");
        Ok(string.as_bytes().to_vec())
    }
    fn proccess_command(&self, mc_server: &mut McServer) -> Result<()> {
        match self {
            Self::Start => {
                mc_server.start_server()?;
            }
            Self::Status => {
                println!("Server Status: {:?}", mc_server.status());
            }
            Self::Stop => {
                mc_server.send_command(self)?;
                mc_server.server_closed();
            }
            _ => mc_server.send_command(self)?,
        };
        Ok(())
    }
}
