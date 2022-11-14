use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::FileTree,
    scaffold::{
        entry_type::definitions::{EntryTypeReference, Referenceable},
        index::IndexType,
    },
};

use super::{build_handlebars, render_template_file_tree_and_merge_with_existing};

#[derive(Serialize)]
pub struct ScaffoldIndexData {
    dna_role_id: String,
    coordinator_zome_manifest: ZomeManifest,
    index_type: IndexType,
    index_name: String,
    referenceable: Referenceable,
}
pub fn scaffold_index_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    dna_role_id: &String,
    coordinator_zome_manifest: &ZomeManifest,
    index_type: &IndexType,
    index_name: &String,
    entry_type_reference: &EntryTypeReference,
) -> ScaffoldResult<FileTree> {
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

    Ok(app_file_tree)
}
