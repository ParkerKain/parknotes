use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    text::Text,
    widgets::{block::*, *},
};

use super::tui::Tui;

pub enum CurrentScreen {
    Main,
    CreatingNote,
    DeletingNote,
    CreatingProject,
    DeletingProject,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub num_notes: u16,
    pub num_projects: u16,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            num_notes: 1000,
            num_projects: 1000,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome to Parknotes ");
        let block = Block::default()
            .title(title.alignment(ratatui::layout::Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let lines = vec![
            Line::from(vec![
                "Found ".into(),
                self.num_notes.to_string().into(),
                " notes across ".into(),
                self.num_projects.to_string().into(),
                " projects!".into(),
            ]),
            Line::from(vec!["What would you like to do?".into()]),
            Line::from(vec!["Create a note".into()]),
            Line::from(vec!["Delete a note".into()]),
            Line::from(vec!["Create a project".into()]),
            Line::from(vec!["Delete a project".into()]),
        ];

        let text = Text::from(lines);
        Paragraph::new(text).block(block).render(area, buf);
    }
}
