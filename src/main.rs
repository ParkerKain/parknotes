use std::fs::create_dir_all;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;

/// Represents all settings the user can set
struct Config {
    /// Where everything will be stored locally
    root_dir: PathBuf,
}

/// Represents a single note files
#[derive(Debug)]
struct Note {
    path: PathBuf,
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
    _get_dir_notes(&config.root_dir, &mut notes);
    return notes;
}

fn _get_dir_notes(base: &PathBuf, notes: &mut Vec<Note>) {
    let contents = read_dir(base).unwrap();
    for curr in contents {
        let curr_file = curr.expect("Failed to read");
        let curr_path = curr_file.path();
        if curr_path.is_dir() {
            _get_dir_notes(&curr_path, notes);
        } else {
            let curr_note = Note { path: curr_path };
            notes.push(curr_note)
        }
    }
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
