use std::fs::create_dir_all;
use std::fs::read_dir;
use std::path::PathBuf;

/// Represents all settings the user can set
struct Config {
    /// Where everything will be stored locally
    root_dir: PathBuf,
}

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
    let contents = read_dir(&config.root_dir).unwrap();
    let mut notes: Vec<Note> = Vec::new();
    for curr in contents {
        let curr_note = Note {
            path: curr.unwrap().path(),
        };
        notes.push(curr_note)
    }
    return notes;
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
}
