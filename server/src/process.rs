use tokio::sync::Mutex;

use std::sync::Arc;

use common::{instructions::Instruction, response::Response};

use crate::{
    error::{Error, Result},
    mc_server::McServer,
};

pub trait Command {
    fn as_command(&self) -> Result<Vec<u8>>;
    async fn proccess_command(&self, mc_server: Arc<Mutex<McServer>>) -> Response;
}

impl Command for Instruction {
    fn as_command(&self) -> Result<Vec<u8>> {
        let mut string = match self {
            Self::SaveAll => String::from("/save-all"),
            Self::Stop => String::from("/stop"),
            Self::Say(msg) => String::from(format!("/say {}", msg)),
            Self::Seed => String::from("/seed"),
            Self::Op(player) => String::from(format!("/op {}", player)),
            Self::Deop(player) => String::from(format!("/deop {}", player)),
            Self::WhitelistAdd(player) => String::from(format!("/whitelist add {}", player)),
            _ => return Err(Error::CommandCreationError),
        };
        string.push_str("\n");
        Ok(string.as_bytes().to_vec())
    }
    async fn proccess_command(&self, mc_server: Arc<Mutex<McServer>>) -> Response {
        let mut locked_mc_server = mc_server.lock().await;
        let res: Result<Response> = match self {
            Self::Help => Ok(Response::new(true, Some("Help".to_string()))),
            Self::Start => locked_mc_server
                .start_server()
                .map(|_| Response::new(true, Some("Server started".to_string()))),
            Self::Status => Ok(Response::new(
                true,
                Some(format!("Server Status: {:?}", locked_mc_server.status()).to_string()),
            )),
            Self::Stop => locked_mc_server
                .stop_server()
                .await
                .map(|_| Response::new(true, Some("Server Stoped".to_string()))),
            _ => locked_mc_server
                .send_command(self)
                .await
                .map(|_| Response::new(true, Some("Message send succesfully".to_string()))),
        };
        match res {
            Ok(response) => response,
            Err(e) => Response::new(false, Some(e.to_string())),
        }
    }
}
