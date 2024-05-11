use crate::structs::{Config, Note, Project};

#[derive(PartialEq)]
pub enum CurrentScreen {
    Search,
    Projects,
    Notes,
    Preview,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub projects: Vec<Project>,
    pub notes: Vec<Note>,
    pub current_selected_project: isize,
    pub current_selected_note: isize,
    pub config: Config,
}

impl App {
    pub fn new(notes: Vec<Note>, projects: Vec<Project>, config: Config) -> App {
        let mut app = App {
            current_screen: CurrentScreen::Search,
            projects,
            notes,
            current_selected_project: 0,
            current_selected_note: 0,
            config,
        };

        app
    }

    pub fn next_screen(self: &mut App) {
        self.current_screen = match self.current_screen {
            CurrentScreen::Search => CurrentScreen::Projects,
            CurrentScreen::Projects => CurrentScreen::Notes,
            CurrentScreen::Notes => CurrentScreen::Preview,
            CurrentScreen::Preview => CurrentScreen::Search,
        };
    }
}
