use std::{
    io::Write,
    process::{Child, ChildStdin, Command as StdCommand, Stdio},
};

use crate::{error::Result, process::Command};

const ARGS: [&str; 4] = ["-Xmx6G", "-jar", "fabric-server-1.20.4.jar", "nogui"];

pub struct McServer {
    _process: Option<Child>,
    child_stdin: Option<ChildStdin>,
    status: ServerStatus,
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
            _process: None,
            child_stdin: None,
            status: ServerStatus::Closed,
        }
    }

    pub fn start_server(&mut self) -> Result<()> {
        self._process = Some(
            StdCommand::new("java")
                .args(ARGS)
                .stdin(Stdio::piped())
                .current_dir("../serverdata/")
                .spawn()?,
        );
        self.child_stdin = self._process.as_mut().unwrap().stdin.take();
        self.status = ServerStatus::Running;
        Ok(())
    }

    pub fn status(&self) -> ServerStatus {
        self.status.clone()
    }

    pub fn server_closed(&mut self) {
        self._process = None;
        self.child_stdin = None;
        self.status = ServerStatus::Closed;
    }

    pub fn send_command(&mut self, command: &impl Command) -> Result<()> {
        match &mut self.child_stdin {
            Some(stdin) => {
                let bytes = command.as_command()?;
                stdin.write_all(&bytes).unwrap();
            }
            None => println!("Minecraft server not running"),
        }
        Ok(())
    }
}
