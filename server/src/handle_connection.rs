use tokio::net::TcpStream;

use std::{
    process::ChildStdin,
    sync::{Arc, Mutex},
};

use common::{instructions::Instruction, message::Message};

use crate::{
    error::{Error, Result},
    process::process_instructions,
};

pub async fn handle_connection(
    stream: TcpStream,
    child_stdin: Arc<Mutex<ChildStdin>>,
) -> Result<()> {
    // Create a buffer to store the incoming data.
    let mut buffer = vec![0; 1024]; // Adjust the buffer size as needed.
    let exit = false;

    loop {
        // Read data into the buffer.

        stream.readable().await?;

        match stream.try_read(&mut buffer) {
            Ok(0) => continue,
            Ok(_n) => {
                let received: Instruction = Message::deser(&buffer);
                let mut locked_child_stdin = child_stdin.lock().unwrap();
                let exit = process_instructions(received, &mut locked_child_stdin)?;
                if exit {
                    break;
                }
                buffer.clear();
            }
            Err(e) => return Err(Error::IOError(e)),
        };
    }

    Ok(())
}
