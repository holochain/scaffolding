use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_to_file_tree, FileTree},
};

static LIT_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/lit");
static SVELTE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/svelte");
static VUE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vue");
static VANILLA_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vanilla");

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
    Svelte,
    Vue,
}

impl ToString for UiFramework {
    fn to_string(&self) -> String {
        match self {
            UiFramework::Vanilla => "vanilla",
            UiFramework::Lit => "lit",
            UiFramework::Svelte => "svelte",
            UiFramework::Vue => "vue",
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
            _ => Err(ScaffoldError::InvalidUiFramework(
                s.to_string(),
                "vanilla, lit, svelte, vue".to_string(),
            )),
        }
    }
}

pub fn guess_or_choose_framework(app_file_tree: &FileTree) -> ScaffoldResult<UiFramework> {
    let ui_package_json_path = PathBuf::from("ui/package.json");
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
    }

    choose_ui_framework()
}

pub fn choose_ui_framework() -> ScaffoldResult<UiFramework> {
    let frameworks = vec!["Vanilla", "Lit", "Svelte", "Vue"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose UI framework:")
        .default(0)
        .items(&frameworks[..])
        .interact()?;

    UiFramework::from_str(frameworks[selection].to_lowercase().as_str())
}

pub fn template_for_ui_framework(framework: &UiFramework) -> ScaffoldResult<FileTree> {
    let dir = match framework {
        UiFramework::Lit => &LIT_TEMPLATES,
        UiFramework::Vanilla => &VANILLA_TEMPLATES,
        UiFramework::Svelte => &SVELTE_TEMPLATES,
        UiFramework::Vue => &VUE_TEMPLATES,
    };

    dir_to_file_tree(dir)
}
