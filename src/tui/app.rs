use std::io;

use ratatui::Frame;

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
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}
