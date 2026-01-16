use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{
        dir_exists, file_content, file_exists, load_directory_into_memory,
        template_dirs_to_file_tree, FileTree,
    },
};

static SVELTE_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/svelte");
static VANILLA_TEMPLATES: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/ui-frameworks/vanilla");

static HEADLESS_TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/headless");
static GENERIC_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/generic");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateType {
    Vanilla,
    Svelte,
    Headless,
    Custom(PathBuf),
}

impl TemplateType {
    /// Gets the non-ANSI escaped name or path of the ui framework
    pub fn name(&self) -> String {
        let name = match self {
            TemplateType::Vanilla => "vanilla",
            TemplateType::Svelte => "svelte",
            TemplateType::Headless => "headless",
            TemplateType::Custom(path) => return format!("{path:?}"),
        };
        name.to_string()
    }

    pub fn check_valid_template(&self) -> ScaffoldResult<()> {
        if file_content(&self.file_tree()?, &PathBuf::from("web-app/README.md.hbs")).is_err() {
            return Err(ScaffoldError::MalformedTemplate(
                "Template does not contain a README.md.hbs file in its \"web-app\" directory"
                    .to_string(),
            ))?;
        }

        Ok(())
    }

    pub fn file_tree(&self) -> ScaffoldResult<FileTree> {
        let ui_framework_dir = match self {
            TemplateType::Vanilla => &VANILLA_TEMPLATES,
            TemplateType::Svelte => &SVELTE_TEMPLATES,
            TemplateType::Headless => &HEADLESS_TEMPLATE,
            TemplateType::Custom(path) => return load_directory_into_memory(path),
        };
        template_dirs_to_file_tree(ui_framework_dir, &GENERIC_TEMPLATES)
    }

    pub fn choose() -> ScaffoldResult<TemplateType> {
        let frameworks = [
            TemplateType::Svelte,
            TemplateType::Vanilla,
            TemplateType::Headless,
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }

    pub fn choose_non_headless() -> ScaffoldResult<TemplateType> {
        let frameworks = [TemplateType::Svelte, TemplateType::Vanilla];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose UI framework: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&frameworks[..])
            .interact()?;
        Ok(frameworks[selection].clone())
    }

    /// Checks whether the custom template'path is a path to a nix store
    pub fn is_nixified_custom_template(&self) -> bool {
        if let TemplateType::Custom(path) = self {
            return path.starts_with("/nix/store/");
        }
        false
    }
}

impl From<PathBuf> for TemplateType {
    fn from(path: PathBuf) -> Self {
        TemplateType::Custom(path)
    }
}

impl TryFrom<&FileTree> for TemplateType {
    type Error = ScaffoldError;

    /// Try to get ui framework from app file tree, if the ui framework cannot be inferred, then
    /// the user will be prompted to choose one via `TemplateType::choose`
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
            if ui_package_json.contains("svelte") {
                return Ok(TemplateType::Svelte);
            } else if !dir_exists(app_file_tree, &PathBuf::from("ui/src")) {
                return Ok(TemplateType::Vanilla);
            }
        }
        TemplateType::choose()
    }
}

impl std::fmt::Display for TemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TemplateType::Vanilla => "vanilla".yellow(),
            TemplateType::Svelte => "svelte".bright_red(),
            TemplateType::Headless => "headless (no ui)".italic(),
            TemplateType::Custom(path) => format!("{path:?}").white(),
        };
        write!(f, "{str}")
    }
}

impl FromStr for TemplateType {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<TemplateType> {
        match s.to_ascii_lowercase().as_str() {
            "vanilla" => Ok(TemplateType::Vanilla),
            "svelte" => Ok(TemplateType::Svelte),
            "headless" => Ok(TemplateType::Headless),
            path_str if PathBuf::from(path_str).exists() => Ok(TemplateType::Custom(path_str.into())),
            value => Err(ScaffoldError::MalformedTemplate(format!(
                "Invalid value: {value}, expected vanilla, svelte, headless or a valid/ existing file path"
            ))),
        }
    }
}

impl Serialize for TemplateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TemplateType::Vanilla => serializer.serialize_str("vanilla"),
            TemplateType::Svelte => serializer.serialize_str("svelte"),
            TemplateType::Headless => serializer.serialize_str("headless"),
            TemplateType::Custom(path) => path
                .to_str()
                .ok_or_else(|| serde::ser::Error::custom("Invalid UTF-8 in path"))
                .and_then(|s| serializer.serialize_str(s)),
        }
    }
}

impl<'de> Deserialize<'de> for TemplateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "vanilla" => Ok(TemplateType::Vanilla),
            "svelte" => Ok(TemplateType::Svelte),
            "headless" => Ok(TemplateType::Headless),
            path_str if PathBuf::from(path_str).exists() => Ok(TemplateType::Custom(path_str.into())),
            value => Err(serde::de::Error::custom(format!(
                "Invalid value: {value}, expected vanilla, svelte, headless or a valid/ existing file path"
            ))),
        }
    }
}
