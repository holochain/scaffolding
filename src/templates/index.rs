use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::{
        entry_type::definitions::{EntryTypeReference, Referenceable},
        index::IndexType,
    },
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldIndexData {
    pub dna_role_id: String,
    pub coordinator_zome_manifest: ZomeManifest,
    pub index_type: IndexType,
    pub index_name: String,
    pub referenceable: Referenceable,
}
pub fn scaffold_index_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    dna_role_id: &String,
    coordinator_zome_manifest: &ZomeManifest,
    index_type: &IndexType,
    index_name: &String,
    entry_type_reference: &EntryTypeReference,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldIndexData {
        dna_role_id: dna_role_id.clone(),
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        index_name: index_name.clone(),
        index_type: index_type.clone(),
        referenceable: Referenceable::EntryType(entry_type_reference.clone()),
    };

    let h = build_handlebars(&template_file_tree)?;

    let field_types_path = PathBuf::from("index");
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
        &template_file_tree,
        &PathBuf::from("index.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
