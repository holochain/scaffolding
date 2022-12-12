use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::ZomeManifest;
use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::entry_type::definitions::Referenceable,
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldLinkTypeData {
    pub dna_role_name: String,
    pub coordinator_zome_manifest: ZomeManifest,
    pub from_referenceable: Referenceable,
    pub to_referenceable: Option<Referenceable>,
    pub delete: bool,
    pub bidireccional: bool,
}
pub fn scaffold_link_type_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    dna_role_name: &String,
    coordinator_zome_manifest: &ZomeManifest,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
    delete: bool,
    bidireccional: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldLinkTypeData {
        dna_role_name: dna_role_name.clone(),
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        from_referenceable: from_referenceable.clone(),
        to_referenceable: to_referenceable.clone(),
        delete,
        bidireccional,
    };

    let h = build_handlebars(&template_file_tree)?;

    let link_type_path = PathBuf::from("link-type");
    let v: Vec<OsString> = link_type_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(link_type_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            link_type_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        &template_file_tree,
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
