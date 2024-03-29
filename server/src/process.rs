use std::{io::Write, process::ChildStdin};

use common::message::Message;

pub fn process_instructions(msg: &Message, child_stdin: &mut ChildStdin) {
    let bytes = msg.instruc.as_command();
    child_stdin.write_all(&bytes).unwrap();
}
