use build_fs_tree::serde::Serialize;
use dialoguer::{theme::ColorfulTheme, Select};
use handlebars::Handlebars;
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_to_file_tree, FileTree},
    generators::index::IndexType,
    templates::{
        register_all_partials_in_dir, register_helpers,
        render_template_file_tree_and_merge_with_existing,
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

pub fn template_for_ui_framework(framework: &UiFramework) -> ScaffoldResult<FileTree> {
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

    let mut h = register_helpers(h);

    let field_types_path = PathBuf::from("field-types");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(field_types_templates) = templates_dir.path(&mut v.iter()) {
        h = register_all_partials_in_dir(h, field_types_templates)?;
    }
    h.register_escape_fn(handlebars::no_escape);

    Ok(h)
}

pub fn scaffold_web_app_ui(
    mut app_file_tree: FileTree,
    templates_file_tree: &FileTree,
    app_name: &String,
) -> ScaffoldResult<FileTree> {
    let data = ScaffoldWebAppData {
        app_name: app_name.clone(),
        holochain_client_version: holochain_client_version(),
    };

    let h = build_handlebars(templates_file_tree)?;

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

fn guess_or_choose_framework(app_file_tree: &FileTree) -> ScaffoldResult<UiFramework> {
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

#[derive(Serialize)]
pub struct ScaffoldEntryTypeData {
    dna_role_id: String,
    coordinator_zome_name: String,
    entry_type: EntryDefinition,
    depends_on: Vec<String>,
}
pub fn scaffold_entry_type_templates(
    mut app_file_tree: FileTree,
    dna_role_id: &String,
    coordinator_zome_name: &String,
    entry_def: &EntryDefinition,
    depends_on: &Vec<String>,
) -> ScaffoldResult<FileTree> {
    let framework = guess_or_choose_framework(&app_file_tree)?;

    let data = ScaffoldEntryTypeData {
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_name: coordinator_zome_name.clone(),
        entry_type: entry_def.clone(),
        depends_on: depends_on.clone(),
    };

    let templates = template_for_ui_framework(&framework)?;

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

#[derive(Serialize)]
pub struct ScaffoldLinkTypeData {
    dna_role_id: String,
    coordinator_zome_name: String,
    from_entry_type: String,
    to_entry_type: String,
}
pub fn scaffold_link_type_templates(
    mut app_file_tree: FileTree,
    dna_role_id: &String,
    coordinator_zome_name: &String,
    from_entry_type: &String,
    to_entry_type: &String,
) -> ScaffoldResult<FileTree> {
    let framework = guess_or_choose_framework(&app_file_tree)?;

    let data = ScaffoldLinkTypeData {
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_name: coordinator_zome_name.clone(),
        from_entry_type: from_entry_type.clone(),
        to_entry_type: to_entry_type.clone(),
    };

    let templates = template_for_ui_framework(&framework)?;

    let h = build_handlebars(&templates)?;

    let field_types_path = PathBuf::from("link-type");
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

#[derive(Serialize)]
pub struct ScaffoldIndexData {
    dna_role_id: String,
    coordinator_zome_name: String,
    index_type: IndexType,
    index_name: String,
    entry_types: Vec<String>,
}
pub fn scaffold_index_templates(
    mut app_file_tree: FileTree,
    dna_role_id: &String,
    coordinator_zome_name: &String,
    index_type: &IndexType,
    index_name: &String,
    entry_types: &Vec<String>,
) -> ScaffoldResult<FileTree> {
    let framework = guess_or_choose_framework(&app_file_tree)?;

    let data = ScaffoldIndexData {
        entry_types: entry_types.clone(),
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_name: coordinator_zome_name.clone(),
        index_name: index_name.clone(),
        index_type: index_type.clone(),
    };

    let templates = template_for_ui_framework(&framework)?;

    let h = build_handlebars(&templates)?;

    let field_types_path = PathBuf::from("index");
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
