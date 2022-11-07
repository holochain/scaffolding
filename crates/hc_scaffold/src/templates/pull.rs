use degit::degit;
use std::path::{Path, PathBuf};
use temp_dir::TempDir;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_exists, load_directory_into_memory, FileTree},
};

use super::template_path;

pub fn get_template(template_url: &String, template: &Option<String>) -> ScaffoldResult<FileTree> {
    let tempdir = TempDir::new().unwrap();

    let tempdir_path = tempdir.path().to_path_buf();
    degit(template_url.as_str(), tempdir_path.to_str().unwrap());

    let mut path = tempdir_path.join(template_path());

    if let Some(t) = template {
        path = path.join(format!(".template.{}", t));
    }

    if !path.as_path().exists() {
        return Err(ScaffoldError::TemplateNotFoundInRepo(path.clone()))?;
    }

    load_directory_into_memory(&path)
}
