use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders},
    Frame,
};

use super::app::App;

pub fn search_ui(f: &mut Frame, chunks: &Rc<[Rect]>) {
    let title = Title::from(" Search ".bold());

    let title_block = Block::default()
        .title(title.alignment(Alignment::Left))
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(title_block, chunks[0]);
}

pub fn projects_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>) {
    let projects_block_title = Title::from(" Projects ".bold());

    let projects_block = Block::default()
        .title(projects_block_title.alignment(Alignment::Left))
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(projects_block, center_chunks[0]);
}
pub fn notes_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>) {
    let notes_block_title = Title::from(" Notes ".bold());
    let notes_block = Block::default()
        .title(notes_block_title.alignment(Alignment::Left))
        .borders(Borders::ALL)
        .style(Style::default());
    f.render_widget(notes_block, center_chunks[1]);
}

pub fn preview_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>) {
    let preview_block_title = Title::from(" Preview ".bold());
    let preview_block = Block::default()
        .title(preview_block_title.alignment(Alignment::Left))
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(preview_block, center_chunks[2]);
}

pub fn center_panel_ui(f: &mut Frame, chunks: &Rc<[Rect]>) {
    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[1]);

    projects_panel(f, &center_chunks);
    notes_panel(f, &center_chunks);
    preview_panel(f, &center_chunks);
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    search_ui(f, &chunks);
    center_panel_ui(f, &chunks);
}
