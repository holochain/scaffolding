use std::path::PathBuf;

use crate::{error::ScaffoldResult, file_tree::FileTree, templates::scaffold_dir};

use super::ScaffoldWebAppData;

pub fn scaffold_vue_web_app(data: &ScaffoldWebAppData) -> ScaffoldResult<FileTree> {
    scaffold_dir(&PathBuf::from("uis/vue/web-app"), data)
}
