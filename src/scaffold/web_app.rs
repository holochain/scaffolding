use build_fs_tree::{dir, file};
use package_manager::PackageManager;
use std::path::PathBuf;

use crate::error::ScaffoldResult;
use crate::reserved_words::check_for_reserved_words;
use crate::templates::web_app::scaffold_web_app_template;
use crate::templates::ScaffoldedTemplate;
use crate::{error::ScaffoldError, file_tree::FileTree};

use super::app::{
    cargo::workspace_cargo_toml,
    git::gitignore,
    manifests::{empty_happ_manifest, web_happ_manifest},
    nix::flake_nix,
};

pub mod package_manager;
pub mod template_type;

pub fn scaffold_web_app(
    app_name: &str,
    description: Option<&str>,
    package_manager: PackageManager,
    skip_nix: bool,
    template_file_tree: &FileTree,
    holo_enabled: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_words(app_name)?;

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
            .insert(
                "flake.nix".into(),
                flake_nix(holo_enabled, &package_manager),
            );
    }

    if package_manager == PackageManager::Pnpm {
        app_file_tree
            .dir_content_mut()
            .ok_or(ScaffoldError::PathNotFound(PathBuf::new()))?
            .insert(
                "pnpm-workspace.yaml".into(),
                file!("packages:\n - ui\n - tests"),
            );
    }

    let scaffold_template_result = scaffold_web_app_template(
        app_file_tree,
        template_file_tree,
        app_name,
        package_manager,
        holo_enabled,
    )?;

    Ok(scaffold_template_result)
}
