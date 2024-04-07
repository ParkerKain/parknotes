use std::{io, slice::Iter};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    text::Text,
    widgets::{block::*, *},
};

use crate::{core::create_objects, structs::Config};

use super::tui::Tui;

/// Defines an Enum for each screen of the app
pub enum CurrentScreen {
    Main,
    SelectProject,
    CreateProject,
}

/// All possible selected options on the main menu
/// TODO: If this was an vector or array or something we could more easily traverse it
#[derive(Debug, PartialEq, Eq)]
pub enum MainScreenOptions {
    CreatingNote,
    DeletingNote,
    CreatingProject,
    DeletingProject,
}

impl MainScreenOptions {
    /// Feels like there should be a better way but this is what I got.
    fn next(&self) -> Self {
        match *self {
            Self::CreatingNote => Self::DeletingNote,
            Self::DeletingNote => Self::CreatingProject,
            Self::CreatingProject => Self::DeletingProject,
            Self::DeletingProject => Self::CreatingNote,
        }
    }

    fn prev(&self) -> Self {
        match *self {
            Self::CreatingNote => Self::DeletingProject,
            Self::DeletingNote => Self::CreatingNote,
            Self::CreatingProject => Self::DeletingNote,
            Self::DeletingProject => Self::CreatingProject,
        }
    }

    fn to_string(&self) -> &str {
        match *self {
            Self::CreatingNote => "Create a Note",
            Self::DeletingNote => "Delete a Note",
            Self::CreatingProject => "Create a Project",
            Self::DeletingProject => "Delete a Project",
        }
    }

    fn iterator() -> Iter<'static, MainScreenOptions> {
        static OPTIONS: [MainScreenOptions; 4] = [
            MainScreenOptions::CreatingNote,
            MainScreenOptions::DeletingNote,
            MainScreenOptions::CreatingProject,
            MainScreenOptions::DeletingProject,
        ];
        OPTIONS.iter()
    }
}

/// This guy defines all state within the TUI
pub struct App {
    /// The Screen we are on
    pub current_screen: CurrentScreen,
    /// The number of notes and projects we see
    pub num_notes: usize,
    pub num_projects: usize,
    /// The Main Screen option we are choosing
    pub selected_menu_option: Option<MainScreenOptions>,
    /// If we are exiting
    pub exit: bool,
}

impl App {
    /// Create a new App State - run at app startup
    pub fn new(config: &Config) -> App {
        let (notes, projects) = create_objects(config);
        App {
            current_screen: CurrentScreen::Main,
            num_notes: notes.len(),
            num_projects: projects.len(),
            selected_menu_option: Some(MainScreenOptions::CreatingNote),
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
            KeyCode::Down => self.next_menu_option(),
            KeyCode::Up => self.prev_menu_option(),
            KeyCode::Enter => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_menu_option(&mut self) {
        if let Some(curr) = &self.selected_menu_option {
            self.selected_menu_option = Some(curr.next());
        };
    }

    fn prev_menu_option(&mut self) {
        if let Some(curr) = &self.selected_menu_option {
            self.selected_menu_option = Some(curr.prev());
        };
    }
}

/// Creates the Text for the main screen
/// Note: I am highly unconfident in my usage of lifetimes here.
fn create_menu_lines<'a>(
    selected_menu_option: &'a MainScreenOptions,
    num_notes: &'a usize,
    num_projects: &'a usize,
) -> Vec<Line<'a>> {
    let mut lines = vec![
        Line::from(vec![
            "Found ".into(),
            num_notes.to_string().into(),
            " notes across ".into(),
            num_projects.to_string().into(),
            " projects!".into(),
        ]),
        Line::from(vec!["What would you like to do?".into()]),
    ];
    for curr_option in MainScreenOptions::iterator() {
        let new_line = match curr_option == selected_menu_option {
            true => Line::from(vec!["> ".into(), curr_option.to_string().into()]),
            false => Line::from(vec![curr_option.to_string().into()]),
        };
        lines.push(new_line);
    }
    lines
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome to Parknotes ");
        let block = Block::default()
            .title(title.alignment(ratatui::layout::Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let selected_menu_option = self.selected_menu_option.as_ref().unwrap();
        let lines = create_menu_lines(selected_menu_option, &self.num_notes, &self.num_projects);

        let text = Text::from(lines);
        Paragraph::new(text).block(block).render(area, buf);
    }
}
