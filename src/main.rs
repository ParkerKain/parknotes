use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;
use std::process::exit;

/// Represents all settings the user can set
struct Config {
    /// Where everything will be stored locally
    root_dir: PathBuf,
}

/// Represents a single note files
#[derive(Debug)]
struct Note {
    full_path: PathBuf,
    trunc_path: PathBuf,
}

#[derive(Debug)]
enum Action {
    Create,
    Delete,
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

/// Creates the core notes vector from the root directory
///
/// # Arguments
///
/// * `config` - a reference to a config object
fn create_note_objects(config: &Config) -> Vec<Note> {
    let mut notes: Vec<Note> = Vec::new();
    _get_dir_notes(&config.root_dir, &mut notes, &config.root_dir);
    return notes;
}

fn _get_dir_notes(base: &PathBuf, notes: &mut Vec<Note>, root_dir: &PathBuf) {
    let contents = read_dir(base).unwrap();
    for curr in contents {
        let curr_file = curr.expect("Failed to read");
        let curr_path = curr_file.path();
        if curr_path.is_dir() {
            _get_dir_notes(&curr_path, notes, root_dir);
        } else {
            let trunc_path = curr_path
                .strip_prefix(root_dir.to_path_buf())
                .unwrap()
                .to_path_buf();
            let curr_note = Note {
                full_path: curr_path,
                trunc_path,
            };
            notes.push(curr_note)
        }
    }
}

/// Prompts the user for the action they want to take
fn prompt_for_action() -> Action {
    let mut input = String::new();
    while !["c", "d"].contains(&input.trim()) {
        input = String::new();
        println!("\nWhat action would you like to take?");
        println!("Options are ... \n\t - (c)reate\n\t - (d)elete");
        stdin().read_line(&mut input).expect("Failed to read line");
    }

    if input.trim() == "c" {
        return Action::Create;
    } else if input.trim() == "d" {
        return Action::Delete;
    } else {
        panic!("Unknown input");
    }
}

fn create_new_note(config: &Config, note_suffix: usize) -> PathBuf {
    let mut note_path = PathBuf::from(&config.root_dir);
    let mut note_name = String::from("new_note_");
    note_name.push_str(&note_suffix.to_string());
    note_name.push_str(".md");
    note_path.push(&note_name);
    if note_path.exists() {
        println!("{} already exists, cancelling ...", note_name);
        exit(0);
    }
    let _ = File::create(&note_path);

    println!("New note created: {}", note_name);
    return note_path;
}

fn prompt_for_note(notes: &Vec<Note>) -> PathBuf {
    let mut input = String::new();
    let mut valid_input_passed: bool = false;
    while !valid_input_passed {
        input = String::new();
        println!("\nWhat file would you like to delete?");
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

    return PathBuf::from(".");
}

fn main() {
    println!("Welcome to clife!");

    let config = Config {
        root_dir: PathBuf::from("/home/parker/.clife"),
    };

    if !detect_root_folder(&config) {
        println!("No clife folder detected at {}", config.root_dir.display());
        create_root_folder(&config);
    }

    let notes = create_note_objects(&config);
    println!("Found {} notes", notes.len());

    let action = prompt_for_action();

    match action {
        Action::Create => {
            let note_path = create_new_note(&config, notes.len());
            let _ = std::process::Command::new("nvim")
                .arg(&note_path.into_os_string())
                .status();
        }
        Action::Delete => {
            let note_path = prompt_for_note(&notes);
        }
        _ => {
            println!("Unknown action")
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
        };
        let result: bool = detect_root_folder(&config);
        assert_eq!(result, true)
    }

    #[test]
    fn test_detect_root_folder_not_exists() {
        let config = Config {
            root_dir: PathBuf::from("~/nonsense_folder_ntuyfwntw/"),
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
        };
        let result: Vec<Note> = create_note_objects(&config);
        assert_eq!(result.len(), 3);
    }
}
