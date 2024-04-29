use std::{ffi::OsString, path::PathBuf};

use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    versions,
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldWebAppData {
    pub app_name: String,
    pub holochain_version: String,
    pub hdk_version: String,
    pub hdi_version: String,
    pub holochain_client_version: String,
    pub hc_spin_version: String,
    pub tryorama_version: String,
    pub holo_enabled: bool,
}

pub fn scaffold_web_app_template(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &str,
    holo_enabled: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldWebAppData {
        app_name: app_name.to_owned(),
        holochain_version: versions::HOLOCHAIN_VERSION.to_owned(),
        hdk_version: versions::HDK_VERSION.to_owned(),
        hdi_version: versions::HDI_VERSION.to_owned(),
        holochain_client_version: versions::HOLOCHAIN_CLIENT_VERSION.to_owned(),
        hc_spin_version: versions::HC_SPIN_VERSION.to_owned(),
        tryorama_version: versions::TRYORAMA_VERSION.to_owned(),
        holo_enabled,
    };

    let h = build_handlebars(template_file_tree)?;

    let field_types_path = PathBuf::from("web-app");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            web_app_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        template_file_tree,
        &PathBuf::from("web-app.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
