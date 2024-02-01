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
    pub app_name: String,
    pub dna_role_name: String,
    pub coordinator_zome_manifest: ZomeManifest,
    pub link_type_name: String,
    pub from_referenceable: Referenceable,
    pub to_referenceable: Option<Referenceable>,
    pub delete: bool,
    pub bidirectional: Option<String>,
}
pub fn scaffold_link_type_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &String,
    dna_role_name: &String,
    coordinator_zome_manifest: &ZomeManifest,
    link_type_name: &String,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
    delete: bool,
    bidirectional: &Option<String>,
    no_ui: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldLinkTypeData {
        app_name: app_name.clone(),
        dna_role_name: dna_role_name.clone(),
        coordinator_zome_manifest: coordinator_zome_manifest.clone(),
        from_referenceable: from_referenceable.clone(),
        link_type_name: link_type_name.clone(),
        to_referenceable: to_referenceable.clone(),
        delete,
        bidirectional: bidirectional.clone(),
    };

    let h = build_handlebars(&template_file_tree)?;

    let link_type_path = PathBuf::from("link-type");
    let v: Vec<OsString> = link_type_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(link_type_template) = template_file_tree.path(&mut v.iter()) {
        // TODO: avoid cloning
        let mut link_type_template = link_type_template.clone();
        if no_ui {
            link_type_template.dir_content_mut().and_then(|v| {
                v.retain(|k, _| k.ne(&OsString::from("ui")));
                Some(v)
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
