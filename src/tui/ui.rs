use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::App;

pub fn search_ui(f: &mut Frame, chunks: &Rc<[Rect]>) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Search for a note",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);
}

pub fn center_panel_ui(f: &mut Frame, chunks: &Rc<[Rect]>) {
    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let projects_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let projects_block_title =
        Paragraph::new(Text::styled("Projects", Style::default().fg(Color::Green)))
            .block(projects_block);

    let notes_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let notes_title =
        Paragraph::new(Text::styled("Notes", Style::default().fg(Color::Green))).block(notes_block);

    f.render_widget(projects_block_title, center_chunks[0]);
    f.render_widget(notes_title, center_chunks[1]);
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    search_ui(f, &chunks);
    center_panel_ui(f, &chunks);
}
