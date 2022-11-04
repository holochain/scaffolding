use degit::degit;
use std::path::{Path, PathBuf};
use temp_dir::TempDir;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_exists, load_directory_into_memory, FileTree},
};

use super::template_path;

pub fn pull_template(
    git_url: &String,
    subdirectory_path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
    let tempdir = TempDir::new().unwrap();

    let tempdir_path = tempdir.path().to_path_buf();
    degit(git_url.as_str(), tempdir_path.to_str().unwrap());

    let mut path = tempdir_path.join(template_path());

    if let Some(p) = subdirectory_path {
        path = path.join(p);
    }

    if !path.as_path().exists() {
        return Err(ScaffoldError::TemplateNotFoundInRepo(path.clone))?;
    }

    load_directory_into_memory(&path)
}
