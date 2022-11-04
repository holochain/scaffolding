use git2::Repository;
use std::path::PathBuf;
use temp_dir::TempDir;

use crate::{
    error::ScaffoldResult,
    file_tree::{load_directory_into_memory, FileTree},
};

use super::template_path;

pub fn pull_template(
    git_url: &String,
    subdirectory_path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
    let tempdir = TempDir::new().unwrap();

    let tempdir_path = tempdir.path().to_path_buf();
    let repo = Repository::clone(git_url, tempdir_path)?;

    let mut path = tempdir_path.join(template_path());

    if let Some(p) = subdirectory_path {
        path = path.join(p);
    }
    load_directory_into_memory(&path)
}
