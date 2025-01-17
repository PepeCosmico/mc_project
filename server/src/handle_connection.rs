use tokio::{net::TcpStream, sync::Mutex};

use std::sync::Arc;

use common::{instructions::Instruction, message::read_message};

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
        let response = instruction.proccess_command(mc_server.clone()).await;
        send_response(&mut stream, &response).await?;
    }
}
