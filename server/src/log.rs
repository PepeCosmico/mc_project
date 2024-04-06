use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::ChildStdout,
    sync::mpsc::Receiver,
};

pub async fn logger(stdout: ChildStdout, mut rx: Receiver<String>) {
    let mut reader = BufReader::new(stdout).lines();
    loop {
        tokio::select! {
            line = reader.next_line() => {
            match line.unwrap() {
                Some(string) => println!("{}", string),
                None => ()
            }
        }
            msg = rx.recv() => {
                match msg {
                    Some(_message) => break,
                    None => ()
                }
            }
        }
    }

    println!("thread ended");
}
