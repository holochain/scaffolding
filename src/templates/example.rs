use serde::Serialize;
use std::{ffi::OsString, path::PathBuf};

use crate::{
    error::ScaffoldResult,
    file_tree::{file_content, FileTree},
    scaffold::example::Example,
    versions,
};

use super::{
    build_handlebars, render_template_file_tree_and_merge_with_existing, ScaffoldedTemplate,
};

#[derive(Serialize)]
pub struct ScaffoldExampleData<'a> {
    pub example: &'a str,
    pub holochain_client_version: &'a str,
    pub hdk_version: &'a str,
    pub hdi_version: &'a str,
}

pub fn scaffold_example(
    mut app_file_tree: FileTree,
    template_file_tree: &FileTree,
    example: &Example,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let data = ScaffoldExampleData {
        example: &example.to_string(),
        holochain_client_version: versions::HOLOCHAIN_CLIENT_VERSION,
        hdk_version: versions::HDK_VERSION,
        hdi_version: versions::HDI_VERSION,
    };
    let h = build_handlebars(template_file_tree)?;

    let example_path = PathBuf::from("example");
    let v: Vec<OsString> = example_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(example_template) = template_file_tree.path(&mut v.iter()) {
        app_file_tree = render_template_file_tree_and_merge_with_existing(
            app_file_tree,
            &h,
            example_template,
            &data,
        )?;
    }

    let next_instructions = match file_content(
        template_file_tree,
        &PathBuf::from("example.instructions.hbs"),
    ) {
        Ok(content) => Some(h.render_template(content.as_str(), &data)?),
        Err(_) => None,
    };

    Ok(ScaffoldedTemplate {
        file_tree: app_file_tree,
        next_instructions,
    })
}
