use std::{
    fs,
    io::{BufRead, BufReader},
};

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
    pub current_preview_line: isize,
    pub current_preview_lines: Vec<String>,
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
            current_preview_line: 0,
            current_preview_lines: vec![],
            config,
        };

        app.load_note(0);
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

    pub fn load_note(self: &mut App, note_i: usize) {
        let open_note =
            fs::File::open(self.notes[note_i].full_path.clone()).expect("Could not parse");

        let note_buf = BufReader::new(open_note);

        let preview_lines = note_buf
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        self.current_preview_lines = preview_lines;
        self.current_preview_line = 0;
    }
}
