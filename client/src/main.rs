use error::Result;
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
    let instruction = Instruction::try_from(&input_vec)?;
    println!("{:?}", instruction);
    Ok(instruction)
}

async fn send_message(stream: &mut TcpStream, msg: &impl Message) -> Result<()> {
    let msg_encoded = msg.ser();
    stream.writable().await?;
    stream.try_write(&msg_encoded)?;
    Ok(())
}
