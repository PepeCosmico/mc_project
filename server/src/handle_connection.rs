use tokio::net::TcpStream;

use std::sync::{Arc, Mutex};

use common::{instructions::Instruction, message::read_message, response::Response};

use crate::{
    error::{Error, Result},
    mc_server::McServer,
    process::Command,
    utils::send_response,
};

pub async fn handle_connection(
    mut stream: TcpStream,
    mc_server: Arc<Mutex<McServer>>,
) -> Result<()> {
    loop {
        let instruction = match read_message::<Instruction>(&mut stream).await {
            Ok(instruc) => instruc,
            Err(e) => return Err(Error::ReadMessageError(e)),
        };
        let res: Result<()>;
        {
            let mut locked_mc_server = mc_server.lock().unwrap();
            res = instruction.proccess_command(&mut locked_mc_server);
        }
        let response = match res {
            Ok(_) => Response::new(true),
            Err(_) => Response::new(false),
        };
        send_response(&mut stream, &response).await?;
    }
}
