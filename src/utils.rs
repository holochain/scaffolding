use std::collections::BTreeMap;
use std::{ffi::OsString, path::PathBuf};

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Input, Select, Validator};

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{dir_content, FileTree};

pub fn choose_directory_path(prompt: &str, app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    let mut chosen_directory: Option<PathBuf> = None;

    let mut current_path = PathBuf::new();

    while chosen_directory.is_none() {
        let mut folders = get_folder_names(&dir_content(app_file_tree, &current_path)?);

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

fn get_folder_names(folder: &BTreeMap<OsString, FileTree>) -> Vec<String> {
    folder
        .iter()
        .filter_map(|(key, val)| {
            if val.dir_content().is_some() {
                key.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect()
}

/// "yes" or "no" input dialog, with the option to specify a recommended answer (yes = true, no = false)
pub fn input_yes_or_no(prompt: &str, recommended: Option<bool>) -> ScaffoldResult<bool> {
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
        .items(&[
            format!("Yes{}", yes_recommended),
            format!("No{}", no_recommended),
        ])
        .interact()?
    {
        1 => Ok(false),
        _ => Ok(true),
    }
}

pub fn input_with_custom_validation<'a, V>(prompt: &str, validator: V) -> ScaffoldResult<String>
where
    V: Validator<String> + 'a,
    V::Err: ToString,
{
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(validator)
        .interact_text()?;

    Ok(input)
}

pub fn input_with_case(prompt: &str, case: Case) -> ScaffoldResult<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), String> {
            match input.is_case(case) {
                false => Err(format!("Input must be {:?} case.", case)),
                true => Ok(()),
            }
        })
        .interact_text()?;

    Ok(input)
}

pub fn input_with_case_and_initial_text(
    prompt: &str,
    case: Case,
    initial_text: &str,
) -> ScaffoldResult<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .with_initial_text(initial_text)
        .validate_with(|input: &String| -> Result<(), String> {
            match input.is_case(case) {
                false => Err(format!("Input must be {:?} case.", case)),
                true => Ok(()),
            }
        })
        .interact_text()?;

    Ok(input)
}

pub fn input_no_whitespace(prompt: &str) -> ScaffoldResult<String> {
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

/// Raises an error if input is not of the appropriate_case
pub fn check_case(input: &str, identifier: &str, case: Case) -> ScaffoldResult<()> {
    match input.is_case(case) {
        true => Ok(()),
        false => Err(ScaffoldError::InvalidStringFormat(format!(
            "{} must be snake_case",
            identifier
        ))),
    }
}

/// Raises an error if input is contains white spaces
pub fn check_no_whitespace(input: &str, identifier: &str) -> ScaffoldResult<()> {
    match input.contains(char::is_whitespace) {
        true => Err(ScaffoldError::InvalidStringFormat(format!(
            "{} must *not* contain whitespaces.",
            identifier
        ))),
        false => Ok(()),
    }
}

pub fn unparse(file: &syn::File) -> String {
    prettyplease::unparse(file).replace("///", "//")
}
