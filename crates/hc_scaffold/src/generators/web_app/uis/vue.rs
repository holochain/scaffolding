use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::file;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use include_dir::{include_dir, Dir};
use json_value_merge::Merge;

use crate::{
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{create_dir_all, FileTree},
    templates::{
        register_all_partials_in_dir, register_case_helpers, register_concat_helper,
        register_partials_helpers, register_ts_type_helper, scaffold_dir,
    },
};

use super::{render_typescript_definition, AddEntryTypeComponentsData, ScaffoldWebAppData};

static VUE_WEB_APP: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/uis/vue/web-app");
static VUE_PARTIALS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/uis/vue/partials");
static CREATE_ENTRY_COMPONENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/uis/vue/create-entry.hbs"
));
static ENTRY_DETAIL_COMPONENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/uis/vue/entry-detail.hbs"
));

pub fn scaffold_vue_web_app(data: &ScaffoldWebAppData) -> ScaffoldResult<FileTree> {
    scaffold_dir(&VUE_PARTIALS, data)
}

pub fn create_entry_component(data: &AddEntryTypeComponentsData) -> ScaffoldResult<String> {
    let h = Handlebars::new();

    let h = register_partials_helpers(h);
    let h = register_concat_helper(h);
    let h = register_case_helpers(h);
    let h = register_ts_type_helper(h);

    let h = register_all_partials_in_dir(h, &VUE_PARTIALS)?;

    let s = h.render_template(CREATE_ENTRY_COMPONENT, data)?;

    Ok(s)
}

pub fn entry_detail_component(data: &AddEntryTypeComponentsData) -> ScaffoldResult<String> {
    let h = Handlebars::new();

    let h = register_partials_helpers(h);
    let h = register_concat_helper(h);
    let h = register_case_helpers(h);
    let h = register_ts_type_helper(h);

    let h = register_all_partials_in_dir(h, &VUE_PARTIALS)?;

    let s = h.render_template(ENTRY_DETAIL_COMPONENT, data)?;

    Ok(s)
}

pub fn add_package_json_dependencies(
    mut app_file_tree: FileTree,
    ui_package_path: &PathBuf,
    entry_def: &EntryDefinition,
) -> ScaffoldResult<FileTree> {
    let package_json_path = ui_package_path.join("package.json");

    let v: Vec<OsString> = package_json_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    let package_json_str = app_file_tree
        .path(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(package_json_path.clone()))?
        .file_content()
        .ok_or(ScaffoldError::PathNotFound(package_json_path.clone()))?
        .clone();
    let mut package_json: serde_json::Value = serde_json::from_str(package_json_str.as_str())?;

    for (_name, field_definition) in entry_def.fields.clone() {
        if let Some(template) = VUE_PARTIALS.get_file(format!(
            "{}/package.json.hbs",
            field_definition.field_type.to_string()
        )) {
            if let Some(contents) = template.contents_utf8() {
                let deps_str = Handlebars::new().render_template(contents, &String::from(""))?;

                let partial_package_json: serde_json::Value =
                    serde_json::from_str(deps_str.as_str())?;

                package_json.merge(partial_package_json);
            }
        }
    }
    let package_json_str = serde_json::to_string_pretty(&package_json)?;

    *app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(package_json_path.clone()))?
        .file_content_mut()
        .ok_or(ScaffoldError::PathNotFound(package_json_path.clone()))? = package_json_str;

    Ok(app_file_tree)
}

pub fn add_entry_components(
    app_file_tree: FileTree,
    ui_package_path: &PathBuf,
    dna_role_id: &String,
    coordinator_zome_name: &String,
    entry_def: &EntryDefinition,
) -> ScaffoldResult<FileTree> {
    let data = AddEntryTypeComponentsData {
        entry_type: entry_def.clone(),
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_name: coordinator_zome_name.clone(),
    };

    let mut app_file_tree =
        add_package_json_dependencies(app_file_tree, ui_package_path, entry_def)?;

    let folder_path = ui_package_path
        .join("src")
        .join(dna_role_id)
        .join(coordinator_zome_name);

    create_dir_all(&mut app_file_tree, &folder_path)?;

    let v: Vec<OsString> = folder_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
        .insert(
            OsString::from(format!(
                "Create{}.vue",
                entry_def.name.to_case(Case::Pascal)
            )),
            file!(create_entry_component(&data)?),
        );
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
        .insert(
            OsString::from(format!(
                "{}Detail.vue",
                entry_def.name.to_case(Case::Pascal)
            )),
            file!(entry_detail_component(&data)?),
        );
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
        .insert(
            OsString::from(format!("{}.ts", entry_def.name.to_case(Case::Kebab))),
            file!(render_typescript_definition(&entry_def)),
        );

    Ok(app_file_tree)
}
