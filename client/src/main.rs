use std::io::{self, Write};

use common::{instructions::Instruction, message::Message};

use crate::{
    error::Result,
    utils::{connect_to_server, print_welcome_msg, send_msg_wait_response},
};

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
        if "exit".to_string() == buffer {
            break;
        }
        match process_input(&buffer) {
            Ok(instruc) => {
                let response = send_msg_wait_response(&mut stream, &instruc).await?;
                if response.is_ok() {
                    println!("{}", response.get_msg().unwrap());
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        };
        buffer.clear();
    }

    Ok(())
}

fn process_input(input: &String) -> Result<impl Message> {
    let input_vec: Vec<&str> = input.split(" ").collect();
    let instruction = Instruction::try_from(&input_vec)?;
    Ok(instruction)
}
