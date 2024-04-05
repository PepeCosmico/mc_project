use tokio::net::TcpStream;

use std::{
    process::ChildStdin,
    sync::{Arc, Mutex},
};

use common::{instructions::Instruction, message::read_message, response::Response};

use crate::{
    error::{Error, Result},
    process::process_instructions,
    utils::send_response,
};

pub async fn handle_connection(
    mut stream: TcpStream,
    child_stdin: Arc<Mutex<Option<ChildStdin>>>,
) -> Result<()> {
    loop {
        let instruction = match read_message::<Instruction>(&mut stream).await {
            Ok(instruc) => instruc,
            Err(e) => return Err(Error::ReadMessageError(e)),
        };
        let res: Result<()>;
        {
            let mut locked_child_stdin = child_stdin.lock().unwrap();
            res = process_instructions(&instruction, &mut locked_child_stdin);
        }
        let response = match res {
            Ok(_) => Response::new(true),
            Err(_) => Response::new(false),
        };
        send_response(&mut stream, &response).await?;
    }
}
