use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, list_state: &mut ListState) {
    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ]);

    let [_, title, main, footer, _] = layout.areas(frame.area());

    let title_text = Paragraph::new("Minecraft Server").centered().bold();
    frame.render_widget(title_text, title);

    let footer_text = Paragraph::new("(press `q` to exit, `j` & `k` to move)").centered();
    frame.render_widget(footer_text, footer);

    let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

    let [actions, responses] = layout.areas(main);

    render_actions(frame, actions, list_state);
    render_responses(frame, responses, &app);
}

fn render_actions(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let actions_list = ["Start Server", "Save Server", "Stop Server"];

    let actions_block = Block::new()
        .title(" ACTIONS ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let list = List::new(actions_list)
        .block(actions_block)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

fn render_responses(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::new()
        .title(" RESPONSES ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let responses: Vec<ListItem> = app
        .responses
        .iter()
        .enumerate()
        .map(|(i, resp)| {
            let content = Line::from(Span::raw(format!("{}: {}", i, resp)));
            ListItem::new(content)
        })
        .collect();

    let responses = List::new(responses).block(block);
    frame.render_widget(responses, area);
}
