use tokio::net::TcpStream;
use utils::connect_to_server;

use std::io::{self, Write};

use common::{instructions::Instruction, message::Message};

use crate::{error::Result, utils::print_welcome_msg};

mod error;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    print_welcome_msg();

    let mut stream = connect_to_server().await?;

    let mut buffer = String::new();
    loop {
        print!("-$ ");
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
