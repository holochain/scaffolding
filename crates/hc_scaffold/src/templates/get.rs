use degit::degit;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};
use temp_dir::TempDir;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_content, dir_exists, load_directory_into_memory, FileTree},
};

use super::{choose_or_get_template, templates_path};

pub fn get_template(
    template_url: &String,
    template: &Option<String>,
) -> ScaffoldResult<(String, FileTree)> {
    let tempdir = TempDir::new().unwrap();

    let tempdir_path = tempdir.path().to_path_buf();
    degit(template_url.as_str(), tempdir_path.to_str().unwrap());

    let file_tree = load_directory_into_memory(&tempdir_path)?;

    let template_name = choose_or_get_template(&file_tree, template)?;
    Ok((
        template_name.clone(),
        FileTree::Directory(dir_content(
            &file_tree,
            &templates_path().join(template_name),
        )?),
    ))
}
