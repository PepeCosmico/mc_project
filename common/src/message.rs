use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{Error, Result};

pub use macros::Message;

pub trait Message: Serialize + for<'de> Deserialize<'de> {
    fn ser(&self) -> Vec<u8>;
    fn deser(encoded: &Vec<u8>) -> Self
    where
        Self: Sized;
}

pub async fn send_message(stream: &mut TcpStream, msg: &impl Message) -> Result<()> {
    let msg_encoded = msg.ser();
    stream.writable().await?;
    println!("hola");
    stream.try_write(&msg_encoded)?;
    Ok(())
}

pub async fn read_message<T>(stream: &mut TcpStream) -> Result<T>
where
    T: Message,
{
    let mut buffer = vec![0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => continue,
            Ok(_n) => return Ok(Message::deser(&buffer)),
            Err(e) => return Err(Error::SendMessageError(e)),
        }
    }
}
