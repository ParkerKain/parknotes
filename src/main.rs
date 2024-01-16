use std::env;
use std::ffi::OsStr;
use std::fs::{create_dir, create_dir_all, read_dir, remove_dir_all, remove_file, File};
use std::path::PathBuf;
use std::process::exit;

use inquire::{validator::Validation, InquireError, Select, Text};

use crate::prompts::{prompt_for_action, prompt_for_new_note_name, prompt_for_note};
use crate::structs::{Action, Config, Note, Project};

mod prompts;
mod structs;

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
            let curr_project = Project { trunc_path };
            projects.push(curr_project);
            _get_dir_objects(&curr_path, notes, projects, root_dir, ignore_dirs);
        } else {
            let curr_note = Note { trunc_path };
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
    return note_path;
}

/// Prompts the user for a note to take action on
///
/// # Arguments
///
/// * `notes` - a reference to the notes vector
/// * `action` - an action to take, only used to prompt the user
fn prompt_for_project(project: &Vec<Project>, action: String) -> PathBuf {
    let options = project
        .iter()
        .map(|note| note.trunc_path.to_str().unwrap())
        .collect();
    let prompt = String::from("What project would you like to ") + &action + &String::from("?");
    let ans: Result<&str, InquireError> = Select::new(&prompt, options).with_page_size(20).prompt();

    match ans {
        Ok(choice) => return PathBuf::from(choice.trim()),
        Err(_) => panic!("There was an error, please try again"),
    }
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

/// Confirms with the user that they want a file to be deleted
///
/// # Arguments
///
/// * `path` - the potential file path to delete
fn confirm_delete(path: &PathBuf, full_path: &PathBuf) {
    if full_path.is_dir() {
        println!(
            "{} is a project, so this will delete all child notes as well, be VERY careful!",
            path.display()
        );
    }
    let options = vec!["Yes", "No"];
    let prompt = String::from("Are you sure you want to delete ")
        + &path.to_string_lossy()
        + &String::from("?");
    let ans: Result<&str, InquireError> = Select::new(&prompt, options)
        .with_starting_cursor(1)
        .prompt();

    match ans {
        Ok(input) => {
            if input.trim() == "No" {
                println!("Cancelling ...");
                exit(0);
            }
        }
        Err(_) => panic!("There was an error, please try again"),
    }
}

/// Deletes the passed PathBuf
///
/// # Arguments:
///
/// * `full_path` - the file path to delete
fn delete(full_path: PathBuf) -> bool {
    println!("Deleting {} ...", full_path.display());
    let result: Result<(), std::io::Error>;
    if full_path.is_dir() {
        result = remove_dir_all(&full_path);
    } else {
        result = remove_file(&full_path);
    }
    match result {
        Ok(()) => {
            println!("{} successfully deleted", full_path.display());
        }
        Err(e) => {
            panic!("Failed to delete: {:?}", e);
        }
    }

    return true;
}

/// Prompts the user for a valid project name
fn prompt_for_project_name() -> String {
    let validator = |name: &str| {
        let invalid_chars = vec![
            '/', '\\', '"', '\'', '*', ';', '-', '?', '[', ']', '(', ')', '~', '!', '$', '{', '}',
            '<', '>', '#', '@', '&', '|', ' ',
        ];
        if name
            .chars()
            .into_iter()
            .any(|curr| invalid_chars.contains(&curr))
        {
            return Ok(Validation::Invalid(
                "Name contains invalid character".into(),
            ));
        } else if name.len() == 0 {
            return Ok(Validation::Invalid("Name is length zero".into()));
        } else {
            return Ok(Validation::Valid);
        }
    };
    let name = Text::new("What would you like to name this project?")
        .with_validator(validator)
        .prompt();

    match name {
        Ok(name) => return name,
        Err(_) => panic!("An error happened when asking for your project, try again later."),
    }
}

fn main() {
    println!("Welcome to parknotes!");

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
    };

    if !detect_root_folder(&config) {
        println!(
            "No parknotes folder detected at {}",
            config.root_dir.display()
        );
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
            let new_note_name = prompt_for_new_note_name();
            let new_note_project = prompt_for_project(&projects, String::from("add this note to"));
            let note_path = create_new_note(&config, new_note_name, new_note_project);
            let _ = std::process::Command::new("nvim")
                .arg(&note_path.into_os_string())
                .status();
        }
        Action::DeleteNote => {
            let note_path = prompt_for_note(&notes, String::from("delete"));
            let mut full_path = config.root_dir.clone();
            full_path.push(&note_path);
            confirm_delete(&note_path, &full_path);
            delete(full_path);
        }
        Action::CreateProject => {
            let project_name = prompt_for_project_name();
            create_new_project(&config, project_name);
        }
        Action::DeleteProject => {
            let project_path = prompt_for_project(&projects, String::from("delete"));
            let mut full_path = config.root_dir.clone();
            full_path.push(&project_path);
            confirm_delete(&project_path, &full_path);
            delete(full_path);
        }
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
                "/home/parker/Documents/projects/parknotes/parknotes/test_data/.parknotes/",
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
