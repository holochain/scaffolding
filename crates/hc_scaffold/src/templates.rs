use build_fs_tree::{file, serde::Serialize};
use handlebars::{Context, Handlebars};
use include_dir::{include_dir, Dir};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{create_dir_all, FileTree};

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn get_templates() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    let templates_dir_map = walk_dir(&TEMPLATES_DIR);

    for (path, content) in templates_dir_map {
        if let Some(e) = path.extension() {
            if e == "hbs" {
                handlebars
                    .register_template_string(
                        path.with_extension("").as_os_str().to_str().unwrap(),
                        content,
                    )
                    .unwrap();
            }
        }
    }

    handlebars
}

pub fn scaffold_file<T: Serialize>(template_path: &PathBuf, data: &T) -> ScaffoldResult<String> {
    let h = get_templates();

    let s = h.render(template_path.as_os_str().to_str().unwrap(), data)?;

    Ok(s)
}

pub fn scaffold_dir<T: Serialize>(template_path: &PathBuf, data: &T) -> ScaffoldResult<FileTree> {
    let h = get_templates();

    let mut file_tree = FileTree::Directory(BTreeMap::new());

    for (name, _template) in h.get_templates() {
        if name.starts_with(template_path.as_os_str().to_str().unwrap()) {
            let mut p: PathBuf = PathBuf::from(name)
                .into_iter()
                .skip(template_path.iter().count())
                .collect();
            let file_name = p.file_name().unwrap().to_os_string();
            p.pop();

            let s = h.render(name, data)?;

            create_dir_all(&mut file_tree, &p)?;

            let v: Vec<OsString> = p.iter().map(|s| s.to_os_string()).collect();
            file_tree
                .path_mut(&mut v.iter())
                .ok_or(ScaffoldError::PathNotFound(p.clone()))?
                .dir_content_mut()
                .ok_or(ScaffoldError::PathNotFound(p.clone()))?
                .insert(file_name.to_os_string(), file!(s));
        }
    }
    Ok(file_tree)
}

fn walk_dir(dir: &Dir<'_>) -> BTreeMap<PathBuf, String> {
    let mut contents: BTreeMap<PathBuf, String> = BTreeMap::new();

    for f in dir.files() {
        if let Some(s) = f.contents_utf8() {
            contents.insert(f.path().to_path_buf(), s.to_string());
        }
    }
    for d in dir.dirs() {
        contents.extend(walk_dir(d));
    }

    contents
}
