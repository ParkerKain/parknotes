use std::env;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::remove_file;
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;
use std::process::exit;

use inquire::InquireError;
use inquire::Select;

/// Represents all settings the user can set
struct Config {
    /// Where everything will be stored locally
    root_dir: PathBuf,
    // directories to ignore
    ignore_dirs: Vec<String>,
}

/// Represents a single note files
#[derive(Debug)]
struct Note {
    // full_path: PathBuf,
    trunc_path: PathBuf,
}

/// Represents a project, or really just a directory
#[derive(Debug)]
struct Project {
    // full_path: PathBuf,
    trunc_path: PathBuf,
}

#[derive(Debug)]
enum Action {
    CreateNote,
    Delete,
    CreateProject,
}

/// Returns if the root dir exists already
///
/// # Arguments
///
/// * `config` - a reference to a config object
fn detect_root_folder(config: &Config) -> bool {
    let exists = config.root_dir.try_exists();
    if exists.is_ok() {
        return exists.unwrap();
    } else {
        panic!("Failed to parse root dir {}", config.root_dir.display());
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
    _get_dir_objects(
        &config.root_dir,
        &mut notes,
        &mut projects,
        &config.root_dir,
        &config.ignore_dirs,
    );
    return (notes, projects);
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
fn _get_dir_objects(
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
        let trunc_path = curr_path
            .strip_prefix(root_dir.to_path_buf())
            .unwrap()
            .to_path_buf();
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
            let curr_project = Project {
                // full_path: curr_path,
                trunc_path,
            };
            projects.push(curr_project);
            _get_dir_objects(&curr_path, notes, projects, root_dir, ignore_dirs);
        } else {
            let curr_note = Note {
                // full_path: curr_path,
                trunc_path,
            };
            notes.push(curr_note)
        }
    }
}

/// Prompts the user for the action they want to take
fn prompt_for_action() -> Action {
    let options = vec!["Create Note", "Delete Note", "Create Project"];
    let ans: Result<&str, InquireError> = Select::new("What action would you like to take?", options).prompt();

    match ans {
        Ok(input) => {
            if input.trim() == "Create Note" {
                return Action::CreateNote;
            } else if input.trim() == "Delete Note" {
                return Action::Delete;
            } else if input.trim() == "Create Project" {
                return Action::CreateProject;
            } else {
                panic!("Unknown input");
            }
        },
        Err(_) => panic!("There was an error, please try again"),
    }

}

/// Creates a new note markdown file
///
/// # Arguments
///
/// * `config` - the config file that controls the run
/// * `note_suffix` - the number of the note to start with as a suffix
fn create_new_note(config: &Config, mut note_suffix: usize) -> PathBuf {
    let mut note_created = false;
    let mut note_path = PathBuf::from(&config.root_dir);
    while !note_created {
        note_path = PathBuf::from(&config.root_dir);
        let mut note_name = String::from("new_note_");
        note_name.push_str(&note_suffix.to_string());
        note_name.push_str(".md");
        note_path.push(&note_name);
        if note_path.exists() {
            println!("{} already exists, trying again ...", note_name);
            note_suffix += 1;
            continue;
        }
        let _ = File::create(&note_path);
        println!("New note created: {}", note_name);
        note_created = true;
    }
    return note_path;
}

/// Prompts the user for a note to take action on
///
/// # Arguments
///
/// * `notes` - a reference to the notes vector
/// * `action` - an action to take, only used to prompt the user
fn prompt_for_note(notes: &Vec<Note>, action: String) -> PathBuf {
    let mut input = String::new();
    let mut valid_input_passed: bool = false;
    while !valid_input_passed {
        input = String::new();
        println!("\nWhat file would you like to {}?", action);
        println!("Options are ... ");
        for note in notes {
            println!("- {:?}", note.trunc_path.as_os_str());
        }
        stdin().read_line(&mut input).expect("Failed to read line");
        if notes
            .iter()
            .any(|e| e.trunc_path.to_str() == Some(&input.as_str().trim()))
        {
            valid_input_passed = true;
        }
    }

    return PathBuf::from(input.trim());
}

/// Confirms with the user that they want a file to be deleted
///
/// # Arguments
///
/// * `path` - the potential file path to delete
fn confirm_delete(path: &PathBuf) {
    let mut input = String::new();
    while !["n", "y"].contains(&input.trim()) {
        input = String::new();
        println!("\nAre you sure you want to delete {}?", path.display());
        println!("Options are ... \n\t- (y)es\n\t- (n)o");
        stdin().read_line(&mut input).expect("Failed to read line");
    }

    if &input.trim() == &"n" {
        println!("Cancelling ...");
        exit(0);
    }
}

/// Deletes the passed PathBuf
///
/// # Arguments:
///
/// * `full_path` - the file path to delete
fn delete(full_path: PathBuf) -> bool {
    println!("Deleting note {} ...", full_path.display());
    let result = remove_file(full_path);
    match result {
        Ok(()) => {
            println!("File successfully deleted");
        }
        Err(e) => {
            panic!("Failed to delete file: {:?}", e);
        }
    }

    return true;
}

/// Prompts the user for a valid project name
fn prompt_for_project_name() -> String {
    let mut input = String::new();
    let mut valid_input = false;
    while !valid_input {
        input = String::new();
        println!("\nWhat would you like to name this project?");
        stdin().read_line(&mut input).expect("Failed to read line");

        // Ensure the input is a valid directory name
        valid_input = validate_project_name(&input);
        if !valid_input {
            println!(
                "Potential project name {} contains invalid characters",
                input
            );
            println!("May only use alphanumerics, '_', and '.'");
        }
    }
    return String::from(input.trim());
}

/// Ensures the passed project_name is a valid directory name
///
/// # Arguments
///
/// * project_name - a reference to the project_name
fn validate_project_name(project_name: &String) -> bool {
    if project_name.trim().len() == 0 {
        return false;
    }

    // Ensure the input is a valid directory name
    let valid_input = project_name
        .trim()
        .chars()
        .all(|c| char::is_alphanumeric(c) || ['_', '.'].contains(&c));
    return valid_input;
}

fn main() {
    println!("Welcome to clife!");

    let root_dir_result = env::var("CLIFE_ROOT_DIR");
    let root_dir: String;
    match root_dir_result {
        Ok(dir) => root_dir = dir,
        Err(_) => {
            panic!("Please set the CLIFE_ROOT_DIR environment variable.")
        }
    }

    let config = Config {
        root_dir: PathBuf::from(root_dir),
        ignore_dirs: vec![String::from(".git")],
    };

    if !detect_root_folder(&config) {
        println!("No clife folder detected at {}", config.root_dir.display());
        create_root_folder(&config);
    }

    let (notes, projects) = create_objects(&config);

    println!(
        "Found {} notes across {} projects!",
        notes.len(),
        projects.len()
    );

    let action = prompt_for_action();

    match action {
        Action::CreateNote => {
            let note_path = create_new_note(&config, notes.len() + 1);
            let _ = std::process::Command::new("nvim")
                .arg(&note_path.into_os_string())
                .status();
        }
        Action::Delete => {
            let note_path = prompt_for_note(&notes, String::from("delete"));
            confirm_delete(&note_path);
            let mut full_path = config.root_dir.clone();
            full_path.push(&note_path);
            delete(full_path);
        }
        Action::CreateProject => {
            let _project_name = prompt_for_project_name();
        } // _ => {
          //     println!("Unknown action")
          // }
    }
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
        assert_eq!(result, true)
    }

    #[test]
    fn test_detect_root_folder_not_exists() {
        let config = Config {
            root_dir: PathBuf::from("~/nonsense_folder_ntuyfwntw/"),
            ignore_dirs: vec![],
        };
        let result: bool = detect_root_folder(&config);
        assert_eq!(result, false)
    }

    #[test]
    fn test_create_note_objects() {
        let config = Config {
            root_dir: PathBuf::from(
                "/home/parker/Documents/projects/clife/clife/test_data/.clife/",
            ),
            ignore_dirs: vec![],
        };
        let (result, _) = create_objects(&config);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_valid_project_name() {
        let valid_names = [
            "test",
            "test_1",
            "my.project",
            ".HELLO.P_Arker_",
            "   hello   ",
        ];

        for name in valid_names {
            assert_eq!(validate_project_name(&String::from(name)), true);
        }
    }

    #[test]
    fn test_invalid_project_name() {
        let invalid_names = ["hello parker", "&parker", "_hello_("];

        for name in invalid_names {
            assert_eq!(validate_project_name(&String::from(name)), false);
        }
    }
}
