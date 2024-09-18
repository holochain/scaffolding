use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_exists, file_exists, template_dirs_to_file_tree, FileTree},
};

static LIT_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/lit");
static SVELTE_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/svelte");
static VUE_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/vue");
static VANILLA_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/vanilla");
static REACT_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/react");

static HEADLESS_TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/headless");
static GENERIC_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/generic");

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
    Svelte,
    Vue,
    React,
    Headless,
}

impl UiFramework {
    /// Gets the non-ANSI escaped name of the ui framework
    pub fn name(&self) -> String {
        let name = match self {
            UiFramework::Vanilla => "vanilla",
            UiFramework::Lit => "lit",
            UiFramework::Svelte => "svelte",
            UiFramework::Vue => "vue",
            UiFramework::React => "react",
            UiFramework::Headless => "headless",
        };
        name.to_string()
    }

    pub fn template_filetree(&self) -> ScaffoldResult<FileTree> {
        let ui_framework_dir = match self {
            UiFramework::Lit => &LIT_TEMPLATES,
            UiFramework::Vanilla => &VANILLA_TEMPLATES,
            UiFramework::Svelte => &SVELTE_TEMPLATES,
            UiFramework::Vue => &VUE_TEMPLATES,
            UiFramework::React => &REACT_TEMPLATES,
            UiFramework::Headless => &HEADLESS_TEMPLATE,
        };
        template_dirs_to_file_tree(ui_framework_dir, &GENERIC_TEMPLATES)
    }

    pub fn choose() -> ScaffoldResult<UiFramework> {
        let frameworks = [
            UiFramework::Lit,
            UiFramework::Svelte,
            UiFramework::Vue,
            UiFramework::React,
            UiFramework::Vanilla,
            UiFramework::Headless,
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }

    pub fn choose_non_vanilla() -> ScaffoldResult<UiFramework> {
        let frameworks = [
            UiFramework::Lit,
            UiFramework::Svelte,
            UiFramework::React,
            UiFramework::Vue,
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }

    pub fn choose_non_headless() -> ScaffoldResult<UiFramework> {
        let frameworks = [
            UiFramework::Lit,
            UiFramework::Svelte,
            UiFramework::React,
            UiFramework::Vue,
            UiFramework::Vanilla,
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }
}

impl TryFrom<&FileTree> for UiFramework {
    type Error = ScaffoldError;

    /// Try to get ui framework from app file tree, if the ui framework cannot be inferred, then
    /// the user will be prompted to choose one via `UiFramework::choose`
    fn try_from(app_file_tree: &FileTree) -> Result<Self, Self::Error> {
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
                .map(|c| c.to_owned())
                .ok_or(ScaffoldError::PathNotFound(ui_package_json_path.clone()))?;
            if ui_package_json.contains("lit") {
                return Ok(UiFramework::Lit);
            } else if ui_package_json.contains("svelte") {
                return Ok(UiFramework::Svelte);
            } else if ui_package_json.contains("vue") {
                return Ok(UiFramework::Vue);
            } else if ui_package_json.contains("react") {
                return Ok(UiFramework::React);
            } else if !dir_exists(app_file_tree, &PathBuf::from("ui/src")) {
                return Ok(UiFramework::Vanilla);
            }
        }
        UiFramework::choose()
    }
}

impl std::fmt::Display for UiFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UiFramework::Vanilla => "vanilla".yellow(),
            UiFramework::Lit => "lit".bright_blue(),
            UiFramework::Svelte => "svelte".bright_red(),
            UiFramework::React => "react".cyan(),
            UiFramework::Vue => "vue".green(),
            UiFramework::Headless => "headless (no ui)".italic(),
        };
        write!(f, "{str}")
    }
}

impl FromStr for UiFramework {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<UiFramework> {
        match s.to_ascii_lowercase().as_str() {
            "vanilla" => Ok(UiFramework::Vanilla),
            "svelte" => Ok(UiFramework::Svelte),
            "vue" => Ok(UiFramework::Vue),
            "react" => Ok(UiFramework::React),
            "lit" => Ok(UiFramework::Lit),
            "headless" => Ok(UiFramework::Headless),
            value => Err(ScaffoldError::MalformedTemplate(format!(
                "Invalid value: {value}, expected vanilla, svelte, vue, lit or headless"
            ))),
        }
    }
}
