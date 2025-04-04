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

// TODO: group some params into a new-type or prefer builder pattern
#[allow(unknown_lints, clippy::too_many_arguments, clippy::manual_inspect)]
pub fn scaffold_collection_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &str,
    dna_role_name: &str,
    coordinator_zome_manifest: &ZomeManifest,
    collection_type: &CollectionType,
    collection_name: &str,
    entry_type_reference: &EntryTypeReference,
    deletable: bool,
    no_ui: bool,
    no_spec: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldCollectionData {
        app_name: app_name.to_owned(),
        dna_role_name: dna_role_name.to_owned(),
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        collection_name: collection_name.to_owned(),
        collection_type: *collection_type,
        referenceable: Referenceable::EntryType(entry_type_reference.clone()),
        deletable,
    };

    let h = build_handlebars(template_file_tree)?;

    let field_types_path = PathBuf::from("collection");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        let mut web_app_template = web_app_template.clone();
        if no_ui {
            web_app_template.dir_content_mut().map(|v| {
                v.retain(|k, _| k != "ui");
                v
            });
        }
        if no_spec {
            web_app_template.dir_content_mut().map(|v| {
                v.retain(|k, _| k != "tests");
                v
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
        template_file_tree,
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
