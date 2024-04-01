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
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
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
        let block = Block::default().title(title.alignment(ratatui::layout::Alignment::Center));

        let text = Text::from(vec![Line::from(vec!["Hello: ".into()])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
