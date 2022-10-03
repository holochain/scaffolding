use holochain_scaffolding_utils::*;
use holochain_scaffolding_utils::file;

fn happ_yaml(app_name: &String) -> String {
    format!(
        r#"
---
manifest_version: "1"
name: {}
description: ~
roles:
"#,
        app_name
    )
}

fn web_happ_yaml(app_name: &String, ui_path_from_root: &String) -> String {
    format!(
        r#"
---
manifest_version: "1"
name: {}
ui:
  bundled: "../{}/dist.zip"
happ_manifest:
  bundled: "./{}.happ"
"#,
        app_name, ui_path_from_root, app_name
    )
}

pub fn generate_happ_workdir(app_name: &String) -> FileTree {
    dir! {
        "happ.yaml" => file!(happ_yaml(app_name))
    }
}

pub fn generate_web_happ_workdir(app_name: &String, ui_path_from_root: &String) -> FileTree {
    dir! {
        "happ.yaml" => file!(happ_yaml(app_name))
        "web-happ.yaml" => file!(web_happ_yaml(app_name, ui_path_from_root))
    }
}
