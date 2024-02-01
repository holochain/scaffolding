use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::{
        collection::CollectionType,
        entry_type::definitions::{EntryTypeReference, Referenceable},
    },
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldCollectionData {
    pub app_name: String,
    pub dna_role_name: String,
    pub coordinator_zome_manifest: ZomeManifest,
    pub collection_type: CollectionType,
    pub collection_name: String,
    pub referenceable: Referenceable,
    pub deletable: bool,
}
pub fn scaffold_collection_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &String,
    dna_role_name: &String,
    coordinator_zome_manifest: &ZomeManifest,
    collection_type: &CollectionType,
    collection_name: &String,
    entry_type_reference: &EntryTypeReference,
    deletable: bool,
    no_ui: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldCollectionData {
        app_name: app_name.clone(),
        dna_role_name: dna_role_name.clone(),
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        collection_name: collection_name.clone(),
        collection_type: collection_type.clone(),
        referenceable: Referenceable::EntryType(entry_type_reference.clone()),
        deletable,
    };

    let h = build_handlebars(&template_file_tree)?;

    let field_types_path = PathBuf::from("collection");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        // TODO: avoid cloning
        let mut web_app_template = web_app_template.clone();
        if no_ui {
            web_app_template.dir_content_mut().and_then(|v| {
                v.retain(|k, _| k.ne(&OsString::from("ui")));
                Some(v)
            });
        }
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            &web_app_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        &template_file_tree,
        &PathBuf::from("collection.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
