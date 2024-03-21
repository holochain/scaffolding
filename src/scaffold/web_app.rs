use build_fs_tree::{dir, file};
use std::{ffi::OsString, path::PathBuf};

use crate::error::ScaffoldResult;
use crate::reserved_words::check_for_reserved_words;
use crate::templates::web_app::scaffold_web_app_template;
use crate::templates::ScaffoldedTemplate;
use crate::{error::ScaffoldError, file_tree::FileTree};

use super::app::{
    cargo::workspace_cargo_toml,
    gitignore::gitignore,
    manifests::{empty_happ_manifest, web_happ_manifest},
    nix::flake_nix,
};

pub mod uis;

fn web_app_skeleton(
    app_name: String,
    description: Option<String>,
    skip_nix: bool,
    template_file_tree: &FileTree,
    holo_enabled: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_words(&app_name)?;

    let mut app_file_tree = dir! {
      ".gitignore" => file!(gitignore())
      "workdir" => dir!{
        "happ.yaml" => file!(empty_happ_manifest(app_name.clone(), description)?)
        "web-happ.yaml" => file!(web_happ_manifest(app_name.clone(), format!("./{}.happ", app_name), String::from("../ui/dist.zip"))?)
      }
      "Cargo.toml" => file!(workspace_cargo_toml())
      "dnas" => dir! {},
    };

    if !skip_nix {
        app_file_tree
            .dir_content_mut()
            .ok_or(ScaffoldError::PathNotFound(PathBuf::new()))?
            .insert(OsString::from("flake.nix"), flake_nix(holo_enabled));
    }

    let mut scaffold_template_result =
        scaffold_web_app_template(app_file_tree, template_file_tree, &app_name, holo_enabled)?;

    scaffold_template_result
        .file_tree
        .dir_content_mut()
        .unwrap()
        .insert(OsString::from("dnas"), dir! {});

    Ok(scaffold_template_result)
}

pub fn scaffold_web_app(
    app_name: String,
    description: Option<String>,
    skip_nix: bool,
    template_file_tree: &FileTree,
    holo_enabled: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let scaffolded_template = web_app_skeleton(
        app_name.clone(),
        description,
        skip_nix,
        template_file_tree,
        holo_enabled,
    )?;
    Ok(ScaffoldedTemplate {
        file_tree: scaffolded_template.file_tree,
        next_instructions: scaffolded_template.next_instructions,
    })
}
