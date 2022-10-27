use build_fs_tree::{dir, file, serde::Serialize};
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{create_dir_all, FileTree},
    versions::holochain_client_version,
};

pub mod lit;
pub mod svelte;
pub mod vanilla;
pub mod vue;

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
    Svelte,
    Vue,
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

pub fn choose_ui_framework() -> ScaffoldResult<UiFramework> {
    let frameworks = vec!["Vanilla", "Lit", "Svelte", "Vue"];
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

#[derive(Serialize)]
pub struct AddEntryTypeComponentsData {
    entry_type: EntryDefinition,
    dna_role_id: String,
    coordinator_zome_name: String,
}

pub fn scaffold_web_app_ui(framework: &UiFramework, app_name: &String) -> ScaffoldResult<FileTree> {
    let data = ScaffoldWebAppData {
        app_name: app_name.clone(),
        holochain_client_version: holochain_client_version(),
    };
    match framework {
        UiFramework::Vanilla => vanilla::scaffold_vanilla_web_app(&data),
        UiFramework::Lit => lit::scaffold_lit_web_app(&data),
        UiFramework::Svelte => svelte::scaffold_svelte_web_app(&data),
        UiFramework::Vue => vue::scaffold_vue_web_app(&data),
    }
}

fn guess_or_choose_ui_package_path() -> PathBuf {
    PathBuf::from("ui")
}

fn guess_or_choose_framework() -> ScaffoldResult<UiFramework> {
    Ok(UiFramework::Lit)
}

pub fn render_typescript_definition(entry_def: &EntryDefinition) -> String {
    let fields_types: Vec<String> = entry_def
        .fields
        .iter()
        .map(|(field_name, field_def)| {
            format!("{}: {};", field_name, field_def.field_type.ts_type())
        })
        .collect();

    format!(
        r#"import {{ ActionHash, AgentPubKey, EntryHash }} from '@holochain/client';

export interface {} {{
  {}
}}
"#,
        entry_def.name.to_case(Case::Pascal),
        fields_types.join("\n")
    )
}

pub fn add_entry_components(
    mut app_file_tree: FileTree,
    entry_def: &EntryDefinition,
    dna_role_id: &String,
    coordinator_zome_name: &String,
) -> ScaffoldResult<FileTree> {
    let ui_package_path = guess_or_choose_ui_package_path();

    let framework = guess_or_choose_framework()?;

    match framework {
        UiFramework::Lit => lit::add_entry_components(
            app_file_tree,
            &ui_package_path,
            &dna_role_id,
            coordinator_zome_name,
            entry_def,
        ),
        UiFramework::Svelte => svelte::add_entry_components(
            app_file_tree,
            &ui_package_path,
            &dna_role_id,
            coordinator_zome_name,
            entry_def,
        ),
        UiFramework::Vue => vue::add_entry_components(
            app_file_tree,
            &ui_package_path,
            &dna_role_id,
            coordinator_zome_name,
            entry_def,
        ),
        UiFramework::Vanilla => Ok(app_file_tree),
    }
}
