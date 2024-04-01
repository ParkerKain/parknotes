use std::path::PathBuf;
use std::{env, io};

use crate::core::{
    create_new_note, create_new_project, create_objects, create_root_folder, delete,
    detect_root_folder,
};
use crate::prompts::{
    confirm_delete, prompt_for_action, prompt_for_new_note_name, prompt_for_note,
    prompt_for_project, prompt_for_project_name,
};
use crate::structs::{Action, Config};
use crate::tui::tui::{restore, setup_tui};

mod core;
mod prompts;
mod structs;
mod tui;

fn main() -> io::Result<()> {
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

    let app_result = setup_tui();
    restore()?;
    app_result

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
