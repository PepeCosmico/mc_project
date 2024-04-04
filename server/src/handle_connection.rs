use tokio::{io::AsyncReadExt, net::TcpStream};

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
    mut stream: TcpStream,
    child_stdin: Arc<Mutex<Option<ChildStdin>>>,
) -> Result<()> {
    // Create a buffer to store the incoming data.
    let mut buffer = vec![0; 1024]; // Adjust the buffer size as needed.

    loop {
        // Read data into the buffer.

        match stream.read(&mut buffer).await {
            Ok(0) => continue,
            Ok(_n) => {
                let received: Instruction = Message::deser(&buffer);
                let mut locked_child_stdin = child_stdin.lock().unwrap();
                process_instructions(&received, &mut locked_child_stdin)?;
                if received == Instruction::Stop {
                    break;
                }
            }
            Err(e) => return Err(Error::IOError(e)),
        };
        buffer = vec![0; 1024];
    }

    Ok(())
}
