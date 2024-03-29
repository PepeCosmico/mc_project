use std::{io::Write, net::TcpStream};

use common::{instructions::Instructions, message::Message};

const LISTENER_PORT: &str = "127.0.0.1:25560";

fn main() {
    let mut stream = TcpStream::connect(LISTENER_PORT).unwrap();
    let msg = Message {
        msg: String::from("Save command"),
        instruc: Instructions::SaveAll,
    };

    let encoded = bincode::serialize(&msg).unwrap();

    let _ = stream.write_all(&encoded);
    let _ = stream.shutdown(std::net::Shutdown::Both);
}
