use std::{ffi::OsString, path::PathBuf, vec};

use dialoguer::{theme::ColorfulTheme, Select, Input};
use inflector::Inflector;

use crate::error::{ScaffoldResult, ScaffoldError};
use crate::file_tree::FileTree;

pub fn choose_directory_path(prompt: &String, app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    let mut chosen_directory: Option<PathBuf> = None;

    let mut current_path = PathBuf::new();

    while let None = chosen_directory {
        let v: Vec<OsString> = current_path.iter().map(|s| s.to_os_string()).collect();
        let mut folders =
            get_folders_names(app_file_tree.path(&mut v.iter()).expect("Can't find path"));

        folders = folders
            .clone()
            .into_iter()
            .map(|s| format!("{}/", s))
            .collect();
        let mut default = 0;

        let path_is_empty = current_path.as_os_str().is_empty();

        if !path_is_empty {
            default = 1;
            folders.insert(0, String::from(".."));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{} Current path: {:?}", prompt, current_path))
            .default(default)
            .items(&folders[..])
            .item("[Select this folder]")
            .report(false)
            .clear(true)
            .interact()?;

        if selection == folders.len() {
            chosen_directory = Some(current_path.clone());
        } else if !path_is_empty && selection == 0 {
            current_path.pop();
        } else {
            let mut folder_name = folders[selection].clone();
            folder_name.pop();
            current_path = current_path.join(folder_name);
        }
    }

    let d = chosen_directory.expect("Couldn't choose directory");

    println!("{} Selected path: {:?}", prompt, current_path);

    Ok(d)
}

fn get_folders_names(file_tree: &FileTree) -> Vec<String> {
    match file_tree {
        FileTree::Directory(c) => c
            .into_iter()
            .filter(|d| d.1.dir_content().is_some())
            .map(|(n, _)| n.to_str().unwrap().to_string())
            .collect(),
        FileTree::File(_) => vec![],
    }
}

/// "yes" or "no" input dialog, with the option to specify a recommended answer (yes = true, no = false)
pub fn input_yes_or_no(prompt: &String, recommended: Option<bool>) -> ScaffoldResult<bool> {
    let mut yes_recommended = "";
    let mut no_recommended = "";

    match recommended {
        Some(true) => yes_recommended = " (recommended)",
        Some(false) => no_recommended = " (recommended)",
        None => (),
    }

    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&[format!("Yes{}", yes_recommended), format!("No{}", no_recommended)])
        .interact()?
    {
        1 => Ok(true),
        _ => Ok(false),
    }
}

pub fn input_snake_case(prompt: &String) -> ScaffoldResult<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.is_snake_case() {
                false => Err("Input must be snake_case."),
                true => Ok(()),
            }
        })
        .interact_text()?;

    Ok(input)
}

pub fn input_no_whitespace(prompt: &String) -> ScaffoldResult<String> {
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.as_str().contains(char::is_whitespace) {
                true => Err("Input must *not* contain white spaces."),
                false => Ok(()),
            }
        })
        .interact_text()?;

    Ok(input)
}

/// Raises an error if input is not snake_case
pub fn check_snake_case(input: String, identifier: &str) -> ScaffoldResult<String> {
    match input.is_snake_case() {
        true => Ok(input),
        false => Err(ScaffoldError::InvalidStringFormat(format!("{} must be snake_case", identifier))),
    }
}

/// Raises an error if input is contains white spaces
pub fn check_no_whitespace(input: String, identifier: &str) -> ScaffoldResult<String> {
    match input.as_str().contains(char::is_whitespace) {
        true => Err(ScaffoldError::InvalidStringFormat(format!("{} must *not* contain whitespaces.", identifier))),
        false => Ok(input),
    }
}