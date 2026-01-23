use build_fs_tree::{dir, file};
use std::path::PathBuf;

use super::app::{
    cargo::workspace_cargo_toml,
    git::gitignore,
    manifests::{empty_happ_manifest, web_happ_manifest},
    nix::flake_nix,
};
use crate::error::ScaffoldResult;
use crate::reserved_words::check_for_reserved_keywords;
use crate::templates::web_app::scaffold_web_app_template;
use crate::templates::ScaffoldedTemplate;
use crate::{error::ScaffoldError, file_tree::FileTree};

pub mod npm;
pub mod template_type;

pub fn scaffold_web_app(
    app_name: &str,
    description: Option<&str>,
    skip_nix: bool,
    template_file_tree: &FileTree,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_keywords(app_name)?;

    let mut app_file_tree = dir! {
      ".gitignore" => file!(gitignore())
      "workdir" => dir!{
        "happ.yaml" => file!(empty_happ_manifest(app_name, description)?)
        "web-happ.yaml" => file!(web_happ_manifest(app_name, format!("./{app_name}.happ"), "../ui/dist.zip".into())?)
      }
      "Cargo.toml" => file!(workspace_cargo_toml())
      "dnas" => dir! {},
    };

    if !skip_nix {
        app_file_tree
            .dir_content_mut()
            .ok_or(ScaffoldError::PathNotFound(PathBuf::new()))?
            .insert("flake.nix".into(), flake_nix());
    }

    let scaffold_template_result =
        scaffold_web_app_template(app_file_tree, template_file_tree, app_name)?;

    Ok(scaffold_template_result)
}
