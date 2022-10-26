use std::str::FromStr;

use build_fs_tree::{dir, serde::Serialize};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
    versions::holochain_client_version,
};

pub mod lit;

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
}

impl FromStr for UiFramework {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<UiFramework> {
        match s {
            "lit" => Ok(UiFramework::Lit),
            "vanilla" => Ok(UiFramework::Vanilla),
            _ => Err(ScaffoldError::InvalidUiFramework(
                s.to_string(),
                "lit, vanilla".to_string(),
            )),
        }
    }
}

pub fn choose_ui_framework() -> ScaffoldResult<UiFramework> {
    let frameworks = vec!["Vanilla", "Lit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose UI framework:")
        .default(0)
        .items(&frameworks[..])
        .interact()?;

    UiFramework::from_str(frameworks[selection].to_lowercase().as_str())
}

#[derive(Serialize)]
pub struct ScaffoldWebAppData {
    app_name: String,
    holochain_client_version: String,
}

pub fn scaffold_web_app_ui(framework: &UiFramework, app_name: &String) -> ScaffoldResult<FileTree> {
    let data = ScaffoldWebAppData {
        app_name: app_name.clone(),
        holochain_client_version: holochain_client_version(),
    };
    match framework {
        UiFramework::Lit => lit::scaffold_lit_web_app(&data),
        _ => Ok(dir! {}),
    }
}
