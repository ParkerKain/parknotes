use std::path::PathBuf;

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
#[derive(Debug)]
pub struct Note {
    pub trunc_path: PathBuf,
}

/// Represents a project, or really just a directory
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Project {
    pub trunc_path: PathBuf,
}

#[derive(Debug)]
pub enum Action {
    CreateNote,
    DeleteNote,
    CreateProject,
    DeleteProject,
}
