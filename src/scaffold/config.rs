use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::ScaffoldResult,
    file_tree::{map_file, FileTree},
};

use super::web_app::template_type::TemplateType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScaffoldConfig {
    pub template: TemplateType,
}

impl ScaffoldConfig {
    /// Gets template config written to the root `package.json` file when the hApp was
    /// originally scaffolded
    pub fn from_package_json_path<P: Into<PathBuf>>(path: P) -> ScaffoldResult<Option<Self>> {
        let package_json_path = path.into().join("package.json");
        let Ok(file) = fs::read_to_string(package_json_path) else {
            return Ok(None);
        };
        let file = serde_json::from_str::<Value>(&file)?;
        if let Some(config) = file.get("hcScaffold") {
            let config = serde_json::from_value(config.to_owned())?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    pub fn write_to_package_json(
        mut web_app_file_tree: FileTree,
        template_type: &TemplateType,
    ) -> ScaffoldResult<FileTree> {
        let config = ScaffoldConfig {
            template: template_type.clone(),
        };
        let package_json_path = PathBuf::from("package.json");

        map_file(&mut web_app_file_tree, &package_json_path, |c| {
            let original_content = c.clone();
            let json = serde_json::from_str::<Value>(&c)?;
            let json = match json {
                Value::Object(mut o) => {
                    o.insert(
                        "hcScaffold".to_owned(),
                        serde_json::to_value(&config).unwrap(),
                    );
                    o
                }
                _ => return Ok(original_content),
            };
            let json = serde_json::to_value(json)?;
            let json = serde_json::to_string_pretty(&json)?;
            Ok(json)
        })?;

        Ok(web_app_file_tree)
    }
}
