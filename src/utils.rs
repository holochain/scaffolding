use std::collections::BTreeMap;
use std::{ffi::OsString, path::PathBuf};

use anyhow::Context;
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

    let dir = chosen_directory.context("Couldn't choose directory")?;

    println!("{prompt} Selected path: {current_path:?}");

    Ok(dir)
}

fn get_folder_names(folder: &BTreeMap<OsString, FileTree>) -> Vec<String> {
    folder
        .iter()
        .filter_map(|(key, val)| {
            if val.dir_content().is_some() {
                return key.to_str().map(|s| s.to_owned());
            }
            None
        })
        .collect()
}

/// "yes" or "no" input dialog, with the option to specify a recommended answer (yes = true, no = false)
pub fn input_yes_or_no(prompt: &str, recommended: Option<bool>) -> ScaffoldResult<bool> {
    let yes_recommended = if recommended == Some(true) {
        " (recommended)"
    } else {
        ""
    };
    let no_recommended = if recommended == Some(false) {
        " (recommended)"
    } else {
        ""
    };

    let items = [
        format!("Yes{}", yes_recommended),
        format!("No{}", no_recommended),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&items)
        .interact()?;

    Ok(selection == 0)
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
            check_case(input, "Input", case).map_err(|e| e.to_string())
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
            check_case(input, "Input", case).map_err(|e| e.to_string())
        })
        .interact_text()?;

    Ok(input)
}

pub fn input_no_whitespace(prompt: &str) -> ScaffoldResult<String> {
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), String> {
            check_no_whitespace(input, "Input").map_err(|e| e.to_string())
        })
        .interact_text()?;

    Ok(input)
}

/// Raises an error if input is not of the appropriate_case
pub fn check_case(input: &str, identifier: &str, case: Case) -> ScaffoldResult<()> {
    if !input.is_case(case) {
        return Err(ScaffoldError::InvalidStringFormat(format!(
            "{identifier} must be snake_case",
        )));
    }
    Ok(())
}

/// Raises an error if input is contains white spaces
pub fn check_no_whitespace(input: &str, identifier: &str) -> ScaffoldResult<()> {
    if input.contains(char::is_whitespace) {
        return Err(ScaffoldError::InvalidStringFormat(format!(
            "{identifier} must *not* contain whitespaces.",
        )));
    }
    Ok(())
}

pub fn unparse(file: &syn::File) -> String {
    add_newlines(&prettyplease::unparse(file).replace("///", "//"))
}

/// Inserts new lines that are stripped out by `syn` during programmatic
/// manipulation of Rust code. Newlines and white spaces are not considered
/// tokens by `syn`, so this function restores them to improve code readability.
fn add_newlines(input: &str) -> String {
    let mut formatted_code = String::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut after_imports = false;
    for (i, line) in lines.iter().enumerate() {
        // Add a newline after the imports block
        if !after_imports && line.trim().is_empty() {
            after_imports = true;
            formatted_code.push_str("\n");
        }
        // Add newlines between #[hdk_extern] annotated functions
        if line.trim().starts_with("#[hdk_extern") && i > 0 {
            formatted_code.push_str("\n");
        }

        // Add newlines between non #[hdk_extern] annoteted functions
        if (line.trim().starts_with("fn") && i > 0) && (!lines[i - 1].starts_with("#[hdk_extern")) {
            formatted_code.push_str("\n");
        }
        // Add newlines between #[derive] annotated structs/enums
        if line.trim().starts_with("#[derive") && i > 0 {
            formatted_code.push_str("\n");
        }
        formatted_code.push_str(line);
        formatted_code.push('\n');
    }
    formatted_code
}
