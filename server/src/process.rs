use std::{io::Write, process::ChildStdin};

use common::instructions::Instruction;

pub fn process_instructions(msg: &Instruction, child_stdin: &mut ChildStdin) {
    let command = msg.as_command();
    child_stdin.write_all(&command).unwrap();
}
