use std::{ffi::OsString, path::PathBuf};

use serde::Serialize;

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldDnaData {
    app_name: String,
    dna_name: String,
}
pub fn scaffold_dna_templates(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    app_name: &String,
    dna_name: &String,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldDnaData {
        app_name: app_name.clone(),
        dna_name: dna_name.clone(),
    };

    let h = build_handlebars(&template_file_tree)?;

    let field_types_path = PathBuf::from("dna");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(web_app_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            web_app_template,
            &data,
        )?;
    }

    let next_instructions =
        match file_content(&template_file_tree, &PathBuf::from("dna.instructions.hbs")) {
            Ok(content) => Some(h.render_template(content.as_str(), &data)?),
            Err(_) => None,
        };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
