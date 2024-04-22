use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, io, path::PathBuf, str::FromStr};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_exists, dir_to_file_tree, file_exists, FileTree},
};

static LIT_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/lit");
static SVELTE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/svelte");
static VUE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vue");
static VANILLA_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vanilla");
static HEADLESS_TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/headless");

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
    Svelte,
    Vue,
    Other(String),
}

impl ToString for UiFramework {
    fn to_string(&self) -> String {
        match self {
            UiFramework::Vanilla => "vanilla",
            UiFramework::Lit => "lit",
            UiFramework::Svelte => "svelte",
            UiFramework::Vue => "vue",
            UiFramework::Other(name) => name,
        }
        .into()
    }
}

impl FromStr for UiFramework {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<UiFramework> {
        match s {
            "vanilla" => Ok(UiFramework::Vanilla),
            "svelte" => Ok(UiFramework::Svelte),
            "vue" => Ok(UiFramework::Vue),
            "lit" => Ok(UiFramework::Lit),
            other => Ok(UiFramework::Other(other.to_owned())),
        }
    }
}

pub fn guess_or_choose_framework(app_file_tree: &FileTree) -> ScaffoldResult<UiFramework> {
    let ui_package_json_path = PathBuf::from("ui/package.json");

    if file_exists(app_file_tree, &ui_package_json_path) {
        let v: Vec<OsString> = ui_package_json_path
            .iter()
            .map(|s| s.to_os_string())
            .collect();
        let ui_package_json = app_file_tree
            .path(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(ui_package_json_path.clone()))?
            .file_content()
            .ok_or(ScaffoldError::PathNotFound(ui_package_json_path.clone()))?
            .clone();

        if ui_package_json.contains("lit") {
            return Ok(UiFramework::Lit);
        } else if ui_package_json.contains("svelte") {
            return Ok(UiFramework::Svelte);
        } else if ui_package_json.contains("vue") {
            return Ok(UiFramework::Vue);
        } else if !dir_exists(app_file_tree, &PathBuf::from("ui/src")) {
            return Ok(UiFramework::Vanilla);
        }
    }
    choose_ui_framework()
}

pub fn choose_ui_framework() -> ScaffoldResult<UiFramework> {
    let frameworks = ["Vue", "Svelte", "Lit", "Vanilla", "headless"];
    let selection = prompt_selection("Choose UI framework:", &frameworks)?;

    UiFramework::from_str(frameworks[selection].to_lowercase().as_str())
}

pub fn choose_non_vanilla_ui_framework() -> ScaffoldResult<UiFramework> {
    let frameworks = ["Vue", "Svelte", "Lit"];
    let selection = prompt_selection("Chooose UI framework:", &frameworks)?;
    UiFramework::from_str(frameworks[selection].to_lowercase().as_str())
}

fn prompt_selection(prompt: &str, choices: &[&str]) -> io::Result<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(choices)
        .interact()
}

pub fn template_for_ui_framework(framework: &UiFramework) -> ScaffoldResult<FileTree> {
    let dir = match framework {
        UiFramework::Lit => &LIT_TEMPLATES,
        UiFramework::Vanilla => &VANILLA_TEMPLATES,
        UiFramework::Svelte => &SVELTE_TEMPLATES,
        UiFramework::Vue => &VUE_TEMPLATES,
        UiFramework::Other(other) if other == "headless" => &HEADLESS_TEMPLATE,
        UiFramework::Other(t) => {
            return Err(ScaffoldError::MalformedTemplate(format!(
                "Inbuilt template {t} not found."
            )));
        }
    };

    dir_to_file_tree(dir)
}
