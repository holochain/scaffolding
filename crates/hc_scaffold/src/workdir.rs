use build_fs_tree::{dir, file};
use holochain_scaffolding_utils::*;
use holochain_types::app::app_manifest_v1::AppManifestV1;
use holochain_types::prelude::AppManifest;
use holochain_types::web_app::web_app_manifest_v1::WebAppManifestV1;
use holochain_types::web_app::{AppManifestLocation, WebUI, WebAppManifest};
use mr_bundle;

use crate::error::ScaffoldResult;

fn empty_happ_yaml(app_name: String, app_description: Option<String>) -> ScaffoldResult<String> {
    let manifest = AppManifest::V1(AppManifestV1 {
        name: app_name,
        description: app_description,
        roles: vec![],
    });
    let s = serde_yaml::to_string(&manifest)?;
    Ok(s)
}
/* 
fn web_happ_yaml(app_name: String, ui_path_from_root: &String) -> ScaffoldResult<String> {
    let manifest = WebAppManifest::V1(WebAppManifestV1 {
        name: app_name,
        ui: WebUI {
            location: mr_bundle::Location::Bundled("".into()),
        },
        happ_manifest: AppManifestLocation {
            location: mr_bundle::Location,
        }
    });
    let s = serde_yaml::to_string(&manifest)?;
    Ok(s)
} */

pub fn generate_happ_workdir(
    app_name: String,
    app_description: Option<String>,
) -> ScaffoldResult<FileTree> {
    Ok(dir! {
        "happ.yaml" => file!(empty_happ_yaml(app_name, app_description)?)
    })
}
/* 
pub fn generate_web_happ_workdir(
    app_name: String,
    app_description: Option<String>,
    ui_path_from_root: &String,
) -> ScaffoldResult<FileTree> {
    Ok(dir! {
        "happ.yaml" => file!(empty_happ_yaml(app_name.clone(), app_description)?)
        "web-happ.yaml" => file!(web_happ_yaml(app_name, ui_path_from_root))
    })
}
 */