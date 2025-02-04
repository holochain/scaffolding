use std::path::PathBuf;

use holochain_types::prelude::{AppManifest, AppManifestCurrentBuilder};
use holochain_types::web_app::{
    AppManifestLocation, WebAppManifest, WebAppManifestCurrentBuilder, WebUI,
};
use mr_bundle::Location;

use crate::error::ScaffoldResult;

pub fn empty_happ_manifest(
    app_name: &str,
    app_description: Option<&str>,
) -> ScaffoldResult<String> {
    let manifest: AppManifest = AppManifestCurrentBuilder::default()
        .name(app_name.to_owned())
        .description(app_description.map(String::from))
        .roles(vec![])
        .build()
        .unwrap()
        .into();

    let s = serde_yaml::to_string(&manifest)?;
    Ok(s)
}

pub fn web_happ_manifest<P: Into<PathBuf>>(
    app_name: &str,
    happ_path: P,
    ui_zip_path: P,
) -> ScaffoldResult<String> {
    let manifest: WebAppManifest = WebAppManifestCurrentBuilder::default()
        .name(app_name.to_owned())
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
