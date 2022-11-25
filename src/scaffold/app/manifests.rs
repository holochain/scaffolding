
use holochain_types::prelude::{AppManifest, AppManifestCurrentBuilder};
use holochain_types::web_app::{
    AppManifestLocation, WebAppManifest, WebAppManifestCurrentBuilder, WebUI,
};
use mr_bundle::Location;

use crate::error::ScaffoldResult;

pub fn empty_happ_manifest(app_name: String, app_description: Option<String>) -> ScaffoldResult<String> {
    let manifest: AppManifest = AppManifestCurrentBuilder::default()
        .name(app_name)
        .description(app_description)
        .roles(vec![])
        .build()
        .unwrap()
        .into();

    let s = serde_yaml::to_string(&manifest)?;
    Ok(s)
}

pub fn web_happ_manifest(app_name: String, happ_path: String, ui_zip_path: String) -> ScaffoldResult<String> {
    let manifest: WebAppManifest = WebAppManifestCurrentBuilder::default()
        .name(app_name.clone())
        .happ_manifest(AppManifestLocation {
            location: Location::Bundled(happ_path.into()),
        })
        .ui(WebUI {
            location: Location::Bundled(ui_zip_path.into()),
        })
        .build()
        .unwrap()
        .into();

    let s = serde_yaml::to_string(&manifest)?;
    Ok(s)
}
