use std::error::Error;
use std::ffi::OsStr;
use std::fs::{create_dir, create_dir_all, read_dir, remove_dir_all, remove_file, File};
use std::path::PathBuf;
use std::{env, fs};

use crate::structs::{Config, Note, Project};
use crate::tui::app::App;
use crate::tui::ui::ui;
use crossterm::event::{self, DisableMouseCapture, Event};
use crossterm::event::{EnableMouseCapture, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::io;
use tui::app::CurrentScreen;

mod prompts;
mod structs;
mod tui;

/// Returns if the root dir exists already
///
/// # Arguments
///
/// * `config` - a reference to a config object
fn detect_root_folder(config: &Config) -> bool {
    let exists = config.root_dir.try_exists();
    match exists.is_ok() {
        true => exists.unwrap(),
        false => panic!("Failed to parse root dir {}", config.root_dir.display()),
    }
}

/// Creates a root folder to store things in
///
/// # Arguments
///
/// * `config` - a reference to a config object
fn create_root_folder(config: &Config) {
    _ = create_dir_all(&config.root_dir);
    println!("{} directory created!", config.root_dir.display());
}

/// Creates the core notes and projects vectors from the root directory
///
/// # Arguments
///
/// * `config` - a reference to a config object
fn create_objects(config: &Config) -> (Vec<Note>, Vec<Project>) {
    let mut notes: Vec<Note> = Vec::new();
    let mut projects: Vec<Project> = Vec::new();
    get_dir_objects(
        &config.root_dir,
        &mut notes,
        &mut projects,
        &config.root_dir,
        &config.ignore_dirs,
    );
    (notes, projects)
}

/// Creates notes and projects from the base directory - recurses through directories
///
/// # Arguments
///
/// * `base` - a reference to the base directory to search
/// * `notes` - The current state of a vector of notes to append to
/// * `projects` - The current state of a vector of projects to append to
/// * `root_dir` - the overall root_dir of the run
/// * `ignore_dirs` - any directories that should be ignored, like .git dirs
fn get_dir_objects(
    base: &PathBuf,
    notes: &mut Vec<Note>,
    projects: &mut Vec<Project>,
    root_dir: &PathBuf,
    ignore_dirs: &Vec<String>,
) {
    let contents = read_dir(base).unwrap();
    for curr in contents {
        let curr_file = curr.expect("Failed to read");
        let curr_path = curr_file.path();
        let trunc_path = curr_path.strip_prefix(root_dir).unwrap().to_path_buf();
        if curr_path.is_dir() {
            // I am ashamed of how this works - split path into parts, then compare against ignored
            // dirs
            let components: Vec<&OsStr> = curr_path
                .components()
                .map(|comp| comp.as_os_str())
                .collect();
            let contains_ignored_dir = components
                .iter()
                .any(|comp| ignore_dirs.contains(&String::from(comp.to_str().unwrap())));
            if contains_ignored_dir {
                continue;
            }
            let curr_project = Project::new(trunc_path);
            projects.push(curr_project);
            get_dir_objects(&curr_path, notes, projects, root_dir, ignore_dirs);
        } else {
            let curr_note = Note {
                filename: trunc_path.file_name().unwrap().to_owned(),
            };
            notes.push(curr_note)
        }
    }
}

/// Creates a new note markdown file
///
/// # Arguments
///
/// * `config` - the config file that controls the run
/// * `orig_note_name` - the original intended name of the new note
fn create_new_note(config: &Config, orig_note_name: String, project_path: PathBuf) -> PathBuf {
    let mut note_created = false;
    let mut note_path = PathBuf::from(&config.root_dir);
    let mut note_suffix = String::from("");
    let mut attempt = 0;
    while !note_created {
        note_path = PathBuf::from(&config.root_dir);
        note_path.push(&project_path);
        let mut note_name = String::from(&orig_note_name);
        note_name.push_str(&note_suffix);
        note_name.push_str(".md");
        note_path.push(&note_name);
        if note_path.exists() {
            println!("{} already exists, trying again ...", note_name);
            attempt += 1;
            note_suffix = String::from("_") + &attempt.to_string();
            continue;
        }
        let _ = File::create(&note_path);
        println!("New note created: {}", note_name);
        note_created = true;
    }
    note_path
}

/// Creates a new project directory
///
/// # Arguments
///
/// * `config` - the config file that controls the run
/// * `orig_note_name` - the original intended name of the new note
fn create_new_project(config: &Config, project_name: String) {
    let mut project_path = PathBuf::from(&config.root_dir);
    project_path.push(&project_name);
    if project_path.exists() {
        println!("{} already exists!", project_name);
        return;
    }
    let _ = create_dir(&project_path);
    println!("New project created: {}", project_name);
}

/// Deletes the passed PathBuf
///
/// # Arguments:
///
/// * `full_path` - the file path to delete
fn delete(full_path: PathBuf) -> bool {
    println!("Deleting {} ...", full_path.display());
    let result: Result<(), std::io::Error> = if full_path.is_dir() {
        remove_dir_all(&full_path)
    } else {
        remove_file(&full_path)
    };
    match result {
        Ok(()) => {
            println!("{} successfully deleted", full_path.display());
        }
        Err(e) => {
            panic!("Failed to delete: {:?}", e);
        }
    }

    true
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Projects => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Tab => app.next_screen(),
                    KeyCode::Down => {
                        let new_index = (app.current_selected_project + 1)
                            % isize::try_from(app.projects.len()).unwrap();
                        app.update_current_selected_project(new_index);
                    }
                    KeyCode::Up => {
                        let new_index = (app.current_selected_project - 1)
                            % isize::try_from(app.projects.len()).unwrap();
                        if new_index < 0 {
                            app.update_current_selected_project(
                                isize::try_from(app.projects.len()).unwrap() - 1,
                            );
                        } else {
                            app.update_current_selected_project(new_index);
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Notes => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Tab => app.next_screen(),
                    KeyCode::Down => {
                        app.current_selected_note = (app.current_selected_note + 1)
                            % isize::try_from(app.projects.len()).unwrap();
                    }
                    KeyCode::Up => {
                        let new_index = (app.current_selected_note - 1)
                            % isize::try_from(app.notes.len()).unwrap();
                        if new_index < 0 {
                            app.current_selected_note =
                                isize::try_from(app.notes.len()).unwrap() - 1;
                        } else {
                            app.current_selected_note = new_index
                        }
                    }
                    _ => {}
                },
                _ => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Tab => {
                        app.next_screen();
                    }
                    _ => {}
                },
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to parknotes!");

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create our base objects
    let root_dir_result = env::var("PARKNOTES_ROOT_DIR");
    let root_dir: String;
    match root_dir_result {
        Ok(dir) => root_dir = dir,
        Err(_) => {
            panic!("Please set PARKNOTES_ROOT_DIR environment variable.")
        }
    }
    let config = Config {
        root_dir: PathBuf::from(root_dir),
        ignore_dirs: vec![String::from(".git"), String::from("bin")],
        menu_scroll_buffer: 10,
    };
    if !detect_root_folder(&config) {
        println!(
            "No parknotes folder detected at {}",
            config.root_dir.display()
        );
        create_root_folder(&config);
    }

    let (notes, projects) = create_objects(&config);

    // create app and run it
    let mut app = App::new(notes, projects, config);
    let _res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())

    // let root_dir_result = env::var("PARKNOTES_ROOT_DIR");
    // let root_dir: String;
    // match root_dir_result {
    //     Ok(dir) => root_dir = dir,
    //     Err(_) => {
    //         panic!("Please set PARKNOTES_ROOT_DIR environment variable.")
    //     }
    // }
    //
    // let config = Config {
    //     root_dir: PathBuf::from(root_dir),
    //     ignore_dirs: vec![String::from(".git"), String::from("bin")],
    // };
    //
    // if !detect_root_folder(&config) {
    //     println!(
    //         "No parknotes folder detected at {}",
    //         config.root_dir.display()
    //     );
    //     create_root_folder(&config);
    // }
    //
    // let (notes, projects) = create_objects(&config);
    //
    // println!(
    //     "Found {} notes across {} projects!",
    //     notes.len(),
    //     projects.len()
    // );
    //
    // let action = prompt_for_action();
    //
    // match action {
    //     Action::CreateNote => {
    //         let new_note_name = prompt_for_new_note_name();
    //         let new_note_project = prompt_for_project(&projects, String::from("add this note to"));
    //         let note_path = create_new_note(&config, new_note_name, new_note_project);
    //         let _ = std::process::Command::new("nvim")
    //             .arg(&note_path.into_os_string())
    //             .status();
    //     }
    //     Action::DeleteNote => {
    //         let note_path = prompt_for_note(&notes, String::from("delete"));
    //         let mut full_path = config.root_dir.clone();
    //         full_path.push(&note_path);
    //         confirm_delete(&note_path, &full_path);
    //         delete(full_path);
    //     }
    //     Action::CreateProject => {
    //         let project_name = prompt_for_project_name();
    //         create_new_project(&config, project_name);
    //     }
    //     Action::DeleteProject => {
    //         let project_path = prompt_for_project(&projects, String::from("delete"));
    //         let mut full_path = config.root_dir.clone();
    //         full_path.push(&project_path);
    //         confirm_delete(&project_path, &full_path);
    //         delete(full_path);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_root_folder_exists() {
        let config = Config {
            root_dir: PathBuf::from("/home"),
            ignore_dirs: vec![],
        };
        let result: bool = detect_root_folder(&config);
        assert!(result)
    }

    #[test]
    fn test_detect_root_folder_not_exists() {
        let config = Config {
            root_dir: PathBuf::from("~/nonsense_folder_ntuyfwntw/"),
            ignore_dirs: vec![],
        };
        let result: bool = detect_root_folder(&config);
        assert!(!result)
    }
}
