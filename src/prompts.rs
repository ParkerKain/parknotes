use std::path::PathBuf;

use inquire::{validator::Validation, InquireError, Select, Text};

use crate::structs::{Action, Note};

/// Prompts the user for the action they want to take
pub fn prompt_for_action() -> Action {
    let options = vec![
        "Create Note",
        "Delete Note",
        "Create Project",
        "Delete Project",
    ];
    let ans: Result<&str, InquireError> =
        Select::new("What action would you like to take?", options).prompt();

    match ans {
        Ok(input) => {
            if input.trim() == "Create Note" {
                return Action::CreateNote;
            } else if input.trim() == "Delete Note" {
                return Action::DeleteNote;
            } else if input.trim() == "Create Project" {
                return Action::CreateProject;
            } else if input.trim() == "Delete Project" {
                return Action::DeleteProject;
            } else {
                panic!("Unknown input");
            }
        }
        Err(_) => panic!("There was an error, please try again"),
    }
}

pub fn prompt_for_new_note_name() -> String {
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
    let name = Text::new("What would you like to name this note?")
        .with_validator(validator)
        .prompt();

    match name {
        Ok(name) => return name,
        Err(_) => panic!("An error happened when asking for your name, try again later."),
    }
}

/// Prompts the user for a note to take action on
///
/// # Arguments
///
/// * `notes` - a reference to the notes vector
/// * `action` - an action to take, only used to prompt the user
pub fn prompt_for_note(notes: &Vec<Note>, action: String) -> PathBuf {
    let options = notes
        .iter()
        .map(|note| note.trunc_path.to_str().unwrap())
        .collect();
    let prompt = String::from("What file would you like to ") + &action + &String::from("?");
    let ans: Result<&str, InquireError> = Select::new(&prompt, options).with_page_size(20).prompt();

    match ans {
        Ok(choice) => return PathBuf::from(choice.trim()),
        Err(_) => panic!("There was an error, please try again"),
    }
}
