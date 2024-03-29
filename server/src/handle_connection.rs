use tokio::net::TcpStream;

use std::{
    process::ChildStdin,
    sync::{Arc, Mutex},
};

use common::{instructions::Instructions, message::Message};

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

    loop {
        // Read data into the buffer.

        stream.readable().await?;

        match stream.try_read(&mut buffer) {
            Ok(0) => continue,
            Ok(_n) => {
                let received: Message = bincode::deserialize(&buffer).unwrap();
                let mut locked_child_stdin = child_stdin.lock().unwrap();
                process_instructions(&received, &mut locked_child_stdin);
                if received.instruc == Instructions::Stop {
                    break;
                }
                buffer.clear();
            }
            Err(e) => return Err(Error::IOError(e)),
        };
    }

    Ok(())
}
