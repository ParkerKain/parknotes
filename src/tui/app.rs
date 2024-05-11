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
    pub current_notes: Vec<isize>,
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
            current_notes: vec![],
            current_selected_project: 0,
            current_selected_note: 0,
            config,
        };

        app.update_current_selected_project(0);
        app
    }

    /// Updates current_selected_project, identifying the notes associated with that project
    pub fn update_current_selected_project(self: &mut App, new_project_index: isize) {
        self.current_selected_project = new_project_index;
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
