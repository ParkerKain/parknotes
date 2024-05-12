use std::{ffi::OsString, path::PathBuf};

/// Represents all settings the user can set
pub struct Config {
    /// Where everything will be stored locally
    pub root_dir: PathBuf,
    // directories to ignore
    pub ignore_dirs: Vec<String>,
    // Controller for when the menu scroll starts
    pub menu_scroll_buffer: u16,
}

/// Represents a single note files
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Note {
    pub filename: OsString,
    pub full_path: PathBuf,
}

/// Represents a project, or really just a directory
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Project {
    pub trunc_path: PathBuf,
    pub notes_indicies: Vec<usize>,
}

impl Project {
    pub fn new(trunc_path: PathBuf) -> Self {
        Project {
            trunc_path,
            notes_indicies: vec![],
        }
    }
}

#[derive(Debug)]
pub enum Action {
    CreateNote,
    DeleteNote,
    CreateProject,
    DeleteProject,
}
