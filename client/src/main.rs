use error::{Error, Result};
use tokio::net::TcpStream;

use std::io::{self, Write};

use common::{instructions::Instruction, message::Message};

mod error;

const LISTENER_PORT: &str = "127.0.0.1:25560";

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect(LISTENER_PORT).await.unwrap();

    let mut buffer = String::new();
    loop {
        print!(" > ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        buffer = buffer.trim().to_string();
        match process_input(&buffer) {
            Ok(intruc) => {
                send_message(&mut stream, &intruc).await?;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        };

        if "exit".to_string() == buffer {
            break;
        }
        buffer.clear();
    }

    Ok(())
}

fn process_input(input: &String) -> Result<impl Message> {
    let input_vec: Vec<&str> = input.split(" ").collect();
    if input_vec.len() < 2 {
        return Err(Error::InvalidInputError);
    }
    let msg = match input_vec[0] {
        "server" => todo!(),
        "mcommand" => {
            let msg_vec = input_vec.split_at(1).1.to_vec();
            Instruction::try_from(&msg_vec)?
        }
        &_ => return Err(Error::InvalidInputError),
    };
    Ok(msg)
}

async fn send_message(stream: &mut TcpStream, msg: &impl Message) -> Result<()> {
    let msg_encoded = bincode::serialize(msg)?;
    stream.writable().await?;
    stream.try_write(&msg_encoded)?;
    Ok(())
}
