use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::Title, Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
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
        .enumerate()
        .map(|(i, project)| {
            let style = if app.current_selected_project == i.try_into().unwrap() {
                Style::default().bg(Color::White).fg(Color::Black)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(vec![Span::styled(
                project.trunc_path.to_string_lossy(),
                style,
            )])
        })
        .collect::<Vec<Line>>();
    let num_lines: isize = projects_lines.len() as isize;
    let buffer: isize = app.config.menu_scroll_buffer as isize;
    let scroll_middle_cutoff: isize = (num_lines - buffer) as isize;

    let scroll_state: u16;
    if app.current_selected_project < buffer {
        scroll_state = 0;
    } else if (buffer <= app.current_selected_project)
        && (app.current_selected_project <= scroll_middle_cutoff)
    {
        scroll_state = (app.current_selected_project - buffer) as u16;
    } else {
        scroll_state = (scroll_middle_cutoff - buffer) as u16;
    };

    let projects_paragraph = Paragraph::new(projects_lines)
        .scroll((scroll_state, 0))
        .block(projects_block);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state =
        ScrollbarState::new(num_lines as usize).position(app.current_selected_project as usize);

    f.render_widget(projects_paragraph, center_chunks[0]);
    f.render_stateful_widget(
        scrollbar,
        center_chunks[0].inner(&Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}
pub fn notes_panel(f: &mut Frame, center_chunks: &Rc<[Rect]>, app: &App) {
    let notes_block_title = Title::from(" Notes ".bold());
    let border_color = get_border_color(app, CurrentScreen::Notes);

    let notes_block = Block::default()
        .title(notes_block_title.alignment(Alignment::Left))
        .border_style(border_color)
        .borders(Borders::ALL)
        .style(Style::default());

    let notes_lines = app
        .notes
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            app.projects[usize::try_from(app.current_selected_project).unwrap()]
                .notes_indicies
                .contains(i)
        })
        .map(|(i, note)| {
            let style = if app.current_selected_note == i.try_into().unwrap() {
                Style::default().bg(Color::White).fg(Color::Black)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(vec![Span::styled(note.filename.to_string_lossy(), style)])
        })
        .collect::<Vec<Line>>();
    let num_lines: isize = notes_lines.len() as isize;
    let buffer: isize = app.config.menu_scroll_buffer as isize;
    let scroll_middle_cutoff: isize = (num_lines - buffer) as isize;

    let scroll_state: u16;
    if app.current_selected_note < buffer {
        scroll_state = 0;
    } else if (buffer <= app.current_selected_note)
        && (app.current_selected_note <= scroll_middle_cutoff)
    {
        scroll_state = (app.current_selected_note - buffer) as u16;
    } else {
        scroll_state = (scroll_middle_cutoff - buffer) as u16;
    };

    let notes_paragraph = Paragraph::new(notes_lines)
        .scroll((scroll_state, 0))
        .block(notes_block);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state =
        ScrollbarState::new(num_lines as usize).position(app.current_selected_note as usize);

    f.render_widget(notes_paragraph, center_chunks[1]);
    f.render_stateful_widget(
        scrollbar,
        center_chunks[1].inner(&Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
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
