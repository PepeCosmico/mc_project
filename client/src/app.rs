use common::instructions::Instruction;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::ListState};
use tokio::net::TcpStream;

use crate::{
    error::Result,
    ui,
    utils::{connect_to_server, send_msg_wait_response},
};

pub struct App {
    should_exit: bool,
    pub responses: Vec<String>,
}

impl App {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut list_state = ListState::default();
        list_state.select_first();

        let mut stream = connect_to_server().await?;

        while !self.should_exit {
            terminal.draw(|frame| ui::draw(frame, &self, &mut list_state))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, &mut stream, &mut list_state).await?;
            };
        }
        Ok(())
    }

    async fn handle_key(
        &mut self,
        key: KeyEvent,
        mut stream: &mut TcpStream,
        list_state: &mut ListState,
    ) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Enter => {
                let instruction = match list_state.selected().unwrap() {
                    0 => Instruction::Start,
                    1 => Instruction::SaveAll,
                    2 => Instruction::Stop,
                    _ => Instruction::Help,
                };

                let response = send_msg_wait_response(&mut stream, &instruction).await?;
                if response.is_ok() {
                    self.responses.push(response.get_msg().unwrap());
                }
            }
            KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
            _ => {}
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            responses: Vec::new(),
        }
    }
}
