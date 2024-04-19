use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::app::App;
use super::app::CurrentScreen;

pub fn get_border_color(app: &App, check_variant: CurrentScreen) -> Style {
    if app.current_screen == check_variant {
        return Style::default().fg(Color::Green);
    }
    Style::default()
}

pub fn search_ui(f: &mut Frame, chunks: &Rc<[Rect]>, app: &App) {
    let title = Title::from(" Search ".bold());

    let border_color = get_border_color(app, CurrentScreen::Search);

    let title_block = Block::default()
        .title(title.alignment(Alignment::Left))
        .border_style(border_color)
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(title_block, chunks[0]);
}

pub fn projects_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>, app: &App) {
    let projects_block_title = Title::from(" Projects ".bold());
    let border_color = get_border_color(app, CurrentScreen::Projects);

    let projects_block = Block::default()
        .title(projects_block_title.alignment(Alignment::Left))
        .border_style(border_color)
        .borders(Borders::ALL)
        .style(Style::default());

    let projects_lines = app
        .projects
        .iter()
        .map(|project| {
            Line::from(vec![Span::styled(
                project.trunc_path.to_string_lossy(),
                Style::default().fg(Color::White),
            )])
        })
        .collect::<Vec<Line>>();

    let projects_paragraph = Paragraph::new(projects_lines).block(projects_block);

    f.render_widget(projects_paragraph, center_chunks[0]);
}
pub fn notes_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>, app: &App) {
    let notes_block_title = Title::from(" Notes ".bold());
    let border_color = get_border_color(app, CurrentScreen::Notes);

    let notes_block = Block::default()
        .title(notes_block_title.alignment(Alignment::Left))
        .border_style(border_color)
        .borders(Borders::ALL)
        .style(Style::default());
    f.render_widget(notes_block, center_chunks[1]);
}

pub fn preview_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>, app: &App) {
    let preview_block_title = Title::from(" Preview ".bold());
    let border_color = get_border_color(app, CurrentScreen::Preview);

    let preview_block = Block::default()
        .title(preview_block_title.alignment(Alignment::Left))
        .border_style(border_color)
        .borders(Borders::ALL)
        .style(Style::default());

    f.render_widget(preview_block, center_chunks[2]);
}

pub fn center_panel_ui(f: &mut Frame, chunks: &Rc<[Rect]>, app: &App) {
    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[1]);

    projects_panel(f, &center_chunks, app);
    notes_panel(f, &center_chunks, app);
    preview_panel(f, &center_chunks, app);
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    search_ui(f, &chunks, app);
    center_panel_ui(f, &chunks, app);
}
