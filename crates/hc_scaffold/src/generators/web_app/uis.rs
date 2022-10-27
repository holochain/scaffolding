use build_fs_tree::{dir, file, serde::Serialize};
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use handlebars::Handlebars;
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{create_dir_all, dir_to_file_tree, FileTree},
    templates::{
        register_all_partials_in_dir, register_case_helpers, register_concat_helper,
        render_template_file_tree, render_template_file_tree_and_merge_with_existing,
    },
    versions::holochain_client_version,
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
    dna_role_id: String,
    coordinator_zome_name: String,
    entry_type: EntryDefinition,
}

fn get_templates(framework: &UiFramework) -> ScaffoldResult<FileTree> {
    let dir = match framework {
        UiFramework::Lit => &LIT_TEMPLATES,
        UiFramework::Vanilla => &VANILLA_TEMPLATES,
        UiFramework::Svelte => &SVELTE_TEMPLATES,
        UiFramework::Vue => &VUE_TEMPLATES,
    };

    dir_to_file_tree(dir)
}

pub fn build_handlebars<'a>(templates_dir: &FileTree) -> ScaffoldResult<Handlebars<'a>> {
    let h = Handlebars::new();

    let h = register_concat_helper(h);
    let mut h = register_case_helpers(h);

    let field_types_path = PathBuf::from("field-types");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(field_types_templates) = templates_dir.path(&mut v.iter()) {
        h = register_all_partials_in_dir(h, field_types_templates)?;
    }

    Ok(h)
}

pub fn scaffold_web_app_ui(
    mut app_file_tree: FileTree,
    framework: &UiFramework,
    app_name: &String,
) -> ScaffoldResult<FileTree> {
    let data = ScaffoldWebAppData {
        app_name: app_name.clone(),
        holochain_client_version: holochain_client_version(),
    };

    let templates = get_templates(framework)?;

    let h = build_handlebars(&templates)?;

    let field_types_path = PathBuf::from("web-app");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = templates.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            web_app_template,
            &data,
        )?;
    }

    Ok(app_file_tree)
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

    let data = AddEntryTypeComponentsData {
        entry_type: entry_def.clone(),
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_name: coordinator_zome_name.clone(),
    };

    let templates = get_templates(&framework)?;

    let h = build_handlebars(&templates)?;

    let field_types_path = PathBuf::from("entry-type");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = templates.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            web_app_template,
            &data,
        )?;
    }

    Ok(app_file_tree)
}
