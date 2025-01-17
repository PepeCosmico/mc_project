use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::ChildStdout,
    sync::mpsc::{Receiver, Sender},
};

pub async fn logger(stdout: ChildStdout, mut rx: Receiver<String>, tx: Sender<String>) {
    let mut reader = BufReader::new(stdout).lines();
    let mut check_log: Option<String> = None;
    loop {
        tokio::select! {
            line = reader.next_line() => {
                match line.unwrap() {
                    Some(string) => {
                        check_for_log(&mut check_log, &string, tx.clone());
                        println!("{}", string);
                    },
                    None => ()
                }
            }
            msg = rx.recv() => {
                match msg {
                    Some(message) => match message.as_ref() {
                        "start" => check_log = Some("For help, type \"help\"".to_string()),
                        "stop" => break,
                        _ => ()
                    },
                    None => ()
                }
            }
        }
    }

    println!("thread ended");
}

fn check_for_log(check_msg: &mut Option<String>, line: &str, sender: Sender<String>) {}
