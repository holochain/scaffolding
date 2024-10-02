use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::{ffi::OsString, path::PathBuf};

use anyhow::Context;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Input, Select, Validator};
use dprint_plugin_typescript::configuration::ConfigurationBuilder;

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

#[inline]
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

#[inline]
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

#[inline]
pub fn input_with_case(prompt: &str, case: Case) -> ScaffoldResult<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), String> {
            check_case(input, "Input", case).map_err(|e| e.to_string())
        })
        .interact_text()?;

    Ok(input)
}

#[inline]
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

#[inline]
pub fn input_no_whitespace(prompt: &str) -> ScaffoldResult<String> {
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), String> {
            check_no_whitespace(input, "Input").map_err(|e| e.to_string())
        })
        .interact_text()?;

    Ok(input)
}

#[inline]
/// Raises an error if input is not of the appropriate_case
pub fn check_case(input: &str, identifier: &str, case: Case) -> ScaffoldResult<()> {
    if !input.is_case(case) {
        return Err(ScaffoldError::InvalidStringFormat(format!(
            "{identifier} must be {case:?} Case",
        )));
    }
    Ok(())
}

#[inline]
/// Raises an error if input is contains white spaces
pub fn check_no_whitespace(input: &str, identifier: &str) -> ScaffoldResult<()> {
    if input.contains(char::is_whitespace) {
        return Err(ScaffoldError::InvalidStringFormat(format!(
            "{identifier} must *not* contain whitespaces.",
        )));
    }
    Ok(())
}

/// Inserts new lines that are stripped out by `syn` during programmatic
/// manipulation of Rust code. Newlines and white spaces are not considered
/// tokens by `syn`, so this function restores them to improve code readability.
#[inline]
pub fn unparse_pretty(code: &syn::File) -> String {
    // replace previously converted line comment to doc comments back to line comments
    let formatted = prettyplease::unparse(code).replace("///", "//");
    let lines = formatted.lines().collect::<Vec<&str>>();
    let mut result = String::new();

    let mut last_line_was_import = false;
    let mut last_line_was_comment = false;
    let mut in_attribute = false;
    let mut in_struct = false;
    let mut in_function = false;
    let mut brace_count = 0;

    // Iterate through the lines, adding extra newlines where needed
    for (i, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        // Check if this line is an import
        let is_import = trimmed_line.starts_with("use ");
        let next_line_is_comment = lines.get(i + 1).unwrap_or(&"").starts_with("//");
        let is_comment = trimmed_line.starts_with("//");

        // Check if we're entering or exiting an attribute
        if trimmed_line.starts_with("#[") {
            in_attribute = true;
        } else if in_attribute && !trimmed_line.ends_with(']') {
            in_attribute = false;
        }

        // Check if we're entering or exiting a struct or function
        if trimmed_line.starts_with("pub struct ") || trimmed_line.starts_with("struct ") {
            in_struct = true;
        } else if trimmed_line.starts_with("pub fn ") || trimmed_line.starts_with("fn ") {
            in_function = true;
        }

        // Count braces to determine when we exit a struct or function
        brace_count += trimmed_line.chars().filter(|&c| c == '{').count() as i32;
        brace_count -= trimmed_line.chars().filter(|&c| c == '}').count() as i32;

        if brace_count == 0 {
            in_struct = false;
            in_function = false;
        }

        // Add an extra newline after the imports section
        if last_line_was_import && !is_import && !in_attribute {
            result.push('\n');
        }

        // Add the current line
        result.push_str(line);
        result.push('\n');

        // Add an extra newline between major items, but not after attributes or within structs/functions
        if !in_attribute
            && !in_struct
            && !in_function
            && !is_comment
            && i + 1 < lines.len()
            && should_add_newline(trimmed_line, lines[i + 1].trim())
        {
            result.push('\n');
        }

        if next_line_is_comment && !last_line_was_comment && !is_comment {
            result.push('\n');
        }

        last_line_was_import = is_import;
        last_line_was_comment = is_comment;
    }

    // Final cleanup: remove any triple (or more) newlines
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    result
}

fn should_add_newline(current: &str, next: &str) -> bool {
    let major_items = [
        "pub struct ",
        "struct ",
        "pub enum ",
        "enum ",
        "pub fn ",
        "fn ",
        "#[",
    ];

    major_items
        .iter()
        .any(|&item| current.starts_with(item) || next.starts_with(item))
}

/// Tries to progrmatically format generated ui code if the file extension matches
/// - ts/js/tsx/jsx
/// - svelte
/// - vue
pub fn format_code<P: Into<PathBuf>>(code: &str, file_name: P) -> ScaffoldResult<String> {
    let file_path: PathBuf = file_name.into();
    let ts_format_config = ConfigurationBuilder::new()
        .line_width(120)
        .indent_width(2)
        .build();

    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
        match extension {
            "ts" | "js" | "tsx" | "jsx" => {
                let formatted_code = dprint_plugin_typescript::format_text(
                    &file_path,
                    None,
                    code.to_owned(),
                    &ts_format_config,
                )
                .map_err(|e| anyhow::anyhow!("Failed to format source code: {e:?}"))?;

                if let Some(value) = formatted_code {
                    return Ok(value);
                }
            }
            "svelte" => {
                let formatted_code = markup_fmt::format_text(
                    code,
                    markup_fmt::Language::Svelte,
                    &Default::default(),
                    |path, raw, _| format_nested(path, extension, raw, &ts_format_config),
                )
                .map_err(|e| anyhow::anyhow!("Failed to format Svelte source code: {e:?}"))?;

                return Ok(formatted_code);
            }
            "vue" => {
                let formatted_code = markup_fmt::format_text(
                    code,
                    markup_fmt::Language::Vue,
                    &Default::default(),
                    |path, raw, _| format_nested(path, extension, raw, &ts_format_config),
                )
                .map_err(|e| anyhow::anyhow!("Failed to format Vue source code: {e:?}"))?;

                return Ok(formatted_code);
            }
            _ => {}
        }
    }

    Ok(code.to_owned())
}

/// Formats ts/js code nested in markup
fn format_nested<'a>(
    path: &Path,
    root_extension: &str,
    raw: &'a str,
    ts_format_config: &dprint_plugin_typescript::configuration::Configuration,
) -> ScaffoldResult<Cow<'a, str>> {
    if let Some(nested_extension) = path.extension().and_then(|ext| ext.to_str()) {
        match (root_extension, nested_extension) {
            ("vue", "ts" | "js") => {
                let formatted_code = dprint_plugin_typescript::format_text(
                    path,
                    None,
                    raw.to_owned(),
                    ts_format_config,
                )
                .map_err(|e| anyhow::anyhow!("Failed to format source code: {e:?}"))?;

                if let Some(value) = formatted_code {
                    return Ok(Cow::Owned(value));
                }
            }
            ("svelte", "ts" | "js" | "tsx" | "jsx") => {
                let formatted_code = dprint_plugin_typescript::format_text(
                    path,
                    None,
                    raw.to_owned(),
                    ts_format_config,
                )
                .map_err(|e| anyhow::anyhow!("Failed to format source code: {e:?}"))?;

                if let Some(value) = formatted_code {
                    return Ok(Cow::Owned(value));
                }
            }
            // Provision to format other nested code i.e css
            _ => {}
        }
    }
    Ok(Cow::Borrowed(raw))
}

/// Runs `cargo fmt` if it's available in the current Rust toolchain otherwise will exit
/// gracefully
pub fn run_cargo_fmt_if_available() -> ScaffoldResult<()> {
    let cargo_fmt_available = Command::new("cargo").arg("fmt").arg("--version").output();

    match cargo_fmt_available {
        Ok(output) if output.status.success() => {
            Command::new("cargo").arg("fmt").status()?;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_typescript_code() {
        let code = "function foo() { console.log('Hello, world!'); }";
        let file_name = "test.ts";
        let result = format_code(code, file_name);
        assert!(result.is_ok());
        let formatted_code = result.unwrap();
        assert_eq!(
            formatted_code,
            "function foo() {\n  console.log(\"Hello, world!\");\n}\n"
        );
    }

    #[test]
    fn test_format_javascript_code() {
        let code = "function foo() { console.log('Hello, world!'); }";
        let file_name = "test.js";
        let result = format_code(code, file_name);
        assert!(result.is_ok());
        let formatted_code = result.unwrap();
        assert_eq!(
            formatted_code,
            "function foo() {\n  console.log(\"Hello, world!\");\n}\n"
        );
    }

    #[test]
    fn test_format_tsx_code() {
        let code = "const foo = () => (<div>Hello, world!</div>);";
        let file_name = "test.tsx";
        let result = format_code(code, file_name);
        assert!(result.is_ok());
        let formatted_code = result.unwrap();
        assert_eq!(
            formatted_code,
            "const foo = () => <div>Hello, world!</div>;\n"
        );
    }

    #[test]
    fn test_format_jsx_code() {
        let code = "const foo = () => (<div>Hello, world!</div>);";
        let file_name = "test.jsx";
        let result = format_code(code, file_name);
        assert!(result.is_ok());
        let formatted_code = result.unwrap();
        assert_eq!(
            formatted_code,
            "const foo = () => <div>Hello, world!</div>;\n"
        );
    }

    #[test]
    fn test_format_vue_code() {
        let code = r#"<template>
<div>{{ message }}</div>
<button>click me</button>
</template>

<script lang="ts">
export default {
  data() {
    return {message: 'Hello, world!'}
  }
};
</script>
"#;
        let file_name = "test.vue";
        let result = format_code(code, file_name);
        assert!(result.is_ok());
        let formatted_code = result.unwrap();
        let expected_output = r#"<template>
  <div>{{ message }}</div>
  <button>click me</button>
</template>

<script lang="ts">
export default {
  data() {
    return { message: "Hello, world!" };
  },
};
</script>
"#;
        assert_eq!(formatted_code, expected_output);
    }

    #[test]
    fn test_format_svelte_code() {
        let code = r#"<script lang="ts">
  let greeting = {message: 'Hello, world!'}
</script>

<div>
<div>{greeting.message}</div>
<button>click me</button>
</div>
"#;
        let file_name = "test.svelte";
        let result = format_code(code, file_name);
        assert!(result.is_ok());
        let formatted_code = result.unwrap();
        let expected_output = r#"<script lang="ts">
let greeting = { message: "Hello, world!" };
</script>

<div>
  <div>{greeting.message}</div>
  <button>click me</button>
</div>
"#;
        assert_eq!(formatted_code, expected_output);
    }
}
