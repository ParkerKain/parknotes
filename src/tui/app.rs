use crate::structs::{Note, Project};

#[derive(PartialEq)]
pub enum CurrentScreen {
    Search,
    Projects,
    Notes,
    Preview,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub notes: Vec<Note>,
    pub projects: Vec<Project>,
    pub current_selected_project: isize,
}

impl App {
    pub fn new(notes: Vec<Note>, projects: Vec<Project>) -> App {
        App {
            current_screen: CurrentScreen::Search,
            notes,
            projects,
            current_selected_project: 0,
        }
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
