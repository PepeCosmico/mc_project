use tokio::sync::Mutex;

use std::sync::Arc;

use common::instructions::Instruction;

use crate::{
    error::{Error, Result},
    mc_server::McServer,
};

pub trait Command {
    fn as_command(&self) -> Result<Vec<u8>>;
    async fn proccess_command(&self, mc_server: Arc<Mutex<McServer>>) -> Result<()>;
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
            Self::Op(player) => String::from(format!("/op {}", player)),
            Self::Deop(player) => String::from(format!("/deop {}", player)),
            Self::WhitelistAdd(player) => String::from(format!("/whitelist add {}", player)),
        };
        string.push_str("\n");
        Ok(string.as_bytes().to_vec())
    }
    async fn proccess_command(&self, mc_server: Arc<Mutex<McServer>>) -> Result<()> {
        let mut locked_mc_server = mc_server.lock().await;
        match self {
            Self::Start => {
                locked_mc_server.start_server()?;
            }
            Self::Status => {
                println!("Server Status: {:?}", locked_mc_server.status());
            }
            Self::Stop => {
                locked_mc_server.stop_server().await?;
            }
            _ => {
                locked_mc_server.send_command(self).await?;
            }
        };
        Ok(())
    }
}
