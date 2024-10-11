use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::entry_type::definitions::{FieldType, Referenceable},
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldLinkTypeData<'a> {
    pub app_name: &'a str,
    pub dna_role_name: &'a str,
    pub coordinator_zome_manifest: ZomeManifest,
    pub link_type_name: &'a str,
    pub from_referenceable: Referenceable,
    pub to_referenceable: Option<Referenceable>,
    pub delete: bool,
    pub bidirectional: Option<&'a str>,
}

// TODO: group some params into a new-type or prefer builder pattern
#[allow(unknown_lints, clippy::too_many_arguments, clippy::manual_inspect)]
pub fn scaffold_link_type_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &str,
    dna_role_name: &str,
    coordinator_zome_manifest: &ZomeManifest,
    link_type_name: &str,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
    delete: bool,
    bidirectional: Option<&str>,
    no_ui: bool,
    no_spec: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldLinkTypeData {
        app_name,
        dna_role_name,
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        from_referenceable: from_referenceable.clone(),
        link_type_name,
        to_referenceable: to_referenceable.clone(),
        delete,
        bidirectional,
    };

    // This is a measure to prevent UI from getting scaffolded for link-types where the base
    // is an ExternalHash since it would expect an <ExternalHash>Detail component to exist
    // which is not possible
    let should_skip_ui_gen = no_ui
        || to_referenceable
            .as_ref()
            .map(|r| r.field_type() == FieldType::ExternalHash)
            .unwrap_or_default();

    let h = build_handlebars(template_file_tree)?;

    let link_type_path = PathBuf::from("link-type");
    let v: Vec<OsString> = link_type_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(link_type_template) = template_file_tree.path(&mut v.iter()) {
        let mut link_type_template = link_type_template.clone();
        if should_skip_ui_gen {
            link_type_template.dir_content_mut().map(|v| {
                v.retain(|k, _| k != "ui");
                v
            });
        }
        if no_spec {
            link_type_template.dir_content_mut().map(|v| {
                v.retain(|k, _| k != "tests");
                v
            });
        }
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            &link_type_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        template_file_tree,
        &PathBuf::from("link-type.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
