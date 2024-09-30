use std::{process::Stdio, time::Duration};

use common::instructions::Instruction;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, Command as TokioCommand},
    sync::mpsc::{self, Sender},
    task::JoinHandle,
    time::sleep,
};

use crate::{error::Result, log::logger, process::Command};

const ARGS: [&str; 4] = ["-Xmx6G", "-jar", "fabric-server-1.20.4.jar", "nogui"];

pub struct McServer {
    pub child: Option<Child>,
    pub child_stdin: Option<ChildStdin>,
    pub log_tread: Option<JoinHandle<()>>,
    pub log_channel_sender: Option<Sender<String>>,
    pub status: ServerStatus,
}

#[derive(Clone, Copy, Debug)]
pub enum ServerStatus {
    Closed,
    Running,
    // TODO Starting,
}

impl McServer {
    pub fn new() -> Self {
        McServer {
            child: None,
            child_stdin: None,
            log_tread: None,
            log_channel_sender: None,
            status: ServerStatus::Closed,
        }
    }

    pub fn start_server(&mut self) -> Result<()> {
        let mut child = TokioCommand::new("java")
            .args(ARGS)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .current_dir("../serverdata/")
            .spawn()?;
        let child_stdin = child.stdin.take();
        let child_stdout = child.stdout.take().unwrap();

        self.child = Some(child);
        self.child_stdin = child_stdin;
        let (tx, rx) = mpsc::channel(32);
        let log_thread = tokio::spawn(async move { logger(child_stdout, rx).await });
        self.log_tread = Some(log_thread);
        self.log_channel_sender = Some(tx);
        self.status = ServerStatus::Running;
        Ok(())
    }

    pub async fn stop_server(&mut self) -> Result<()> {
        self.send_command(&Instruction::Stop).await?;
        loop {
            {
                match self.child.as_mut().unwrap().try_wait() {
                    Ok(Some(status)) => {
                        println!("Exited with status: {}", status);
                        let sender = self.log_channel_sender.clone();
                        sender.unwrap().try_send(status.to_string()).unwrap();
                        break;
                    }
                    Ok(None) => (),
                    Err(e) => return Err(e.into()),
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }

    pub fn status(&self) -> ServerStatus {
        self.status.clone()
    }

    pub async fn send_command(&mut self, command: &impl Command) -> Result<()> {
        match &mut self.child_stdin {
            Some(stdin) => {
                let bytes = command.as_command()?;
                let _ = stdin.write_all(&bytes).await?;
            }
            None => println!("Minecraft server not running"),
        }
        Ok(())
    }
}
