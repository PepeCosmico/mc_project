use error::Result;
use tokio::net::TcpStream;

use std::io::{self, Write};

use common::{instructions::Instructions, message::Message};

mod error;

const LISTENER_PORT: &str = "127.0.0.1:25560";

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect(LISTENER_PORT).await;

    let mut buffer = String::new();
    loop {
        print!(" > ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        buffer = buffer.trim().to_string();
        let res = process_input(buffer.clone());

        if "exit".to_string() == buffer {
            break;
        }
        buffer.clear();
    }
}

fn process_input(input: String) -> Result<()> {
    let input_vec: Vec<&str> = input.split(" ").collect();
    let instruction = Instructions::try_from(&input_vec)?;
    println!("{:?}", &instruction);
    Ok(())
}
