use std::{ffi::OsString, path::PathBuf};

use holochain_types::dna::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldCoordinatorZomeData {
    pub dna_role_name: String,
    pub zome_manifest: ZomeManifest,
}

pub fn scaffold_coordinator_zome_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    dna_role_name: &str,
    zome_manifest: &ZomeManifest,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldCoordinatorZomeData {
        dna_role_name: dna_role_name.to_owned(),
        zome_manifest: zome_manifest.clone(),
    };

    let h = build_handlebars(template_file_tree)?;

    let coordinator_zome_path = PathBuf::from("coordinator-zome");
    let v: Vec<OsString> = coordinator_zome_path
        .iter()
        .map(|s| s.to_os_string())
        .collect();

    if let Some(coordinator_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            coordinator_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        template_file_tree,
        &PathBuf::from("coordinator-zome.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
