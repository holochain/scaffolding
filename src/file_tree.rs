use anyhow::Context;
use build_fs_tree::{dir, file, Build, FileSystemTree, MergeableFileSystemTree};
use ignore::WalkBuilder;
use include_dir::Dir;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::utils::unparse_pretty;

pub type FileTree = FileSystemTree<OsString, String>;

// Loads the directory tree in the given path into memory recursively
pub fn load_directory_into_memory(path: &Path) -> ScaffoldResult<FileTree> {
    let mut file_tree: FileTree = dir! {};

    for result in WalkBuilder::new(path).hidden(false).build() {
        let dir_entry = result?
            .path()
            .iter()
            .skip(path.components().count())
            .collect::<PathBuf>();

        if fs::metadata(&path.join(&dir_entry))?.is_dir() {
            create_dir_all(&mut file_tree, &dir_entry)?;
        } else if let Ok(contents) = fs::read_to_string(&path.join(&dir_entry)) {
            insert_file(&mut file_tree, &dir_entry, &contents)?;
        }
    }

    Ok(file_tree)
}

pub fn dir_content(
    file_tree: &FileTree,
    folder_path: &Path,
) -> ScaffoldResult<BTreeMap<OsString, FileTree>> {
    let v: Vec<OsString> = folder_path.iter().map(|s| s.to_os_string()).collect();
    file_tree
        .path(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(folder_path.to_path_buf()))?
        .dir_content()
        .ok_or(ScaffoldError::PathNotFound(folder_path.to_path_buf()))
        .cloned()
}

pub fn dir_exists(app_file_tree: &FileTree, dir_path: &Path) -> bool {
    dir_content(app_file_tree, dir_path).is_ok()
}

pub fn file_exists(app_file_tree: &FileTree, file_path: &Path) -> bool {
    file_content(app_file_tree, file_path).is_ok()
}

pub fn file_content(file_tree: &FileTree, file_path: &Path) -> ScaffoldResult<String> {
    let v: Vec<OsString> = file_path.iter().map(|s| s.to_os_string()).collect();
    file_tree
        .path(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(file_path.to_path_buf()))?
        .file_content()
        .ok_or(ScaffoldError::PathNotFound(file_path.to_path_buf()))
        .cloned()
}

pub fn map_file<F: Fn(String) -> Result<String, ScaffoldError>>(
    file_tree: &mut FileTree,
    file_path: &Path,
    map_fn: F,
) -> ScaffoldResult<()> {
    let contents = file_content(file_tree, file_path)?;
    insert_file(file_tree, file_path, &map_fn(contents)?)
}

pub fn insert_file(
    file_tree: &mut FileTree,
    file_path: &Path,
    content: &str,
) -> ScaffoldResult<()> {
    let mut folder_path = file_path.to_path_buf();
    folder_path.pop();

    insert_file_tree_in_dir(
        file_tree,
        &folder_path,
        (
            file_path.file_name().unwrap().to_os_string(),
            file!(content),
        ),
    )
}

pub fn insert_file_tree_in_dir(
    file_tree: &mut FileTree,
    folder_path: &Path,
    file_tree_to_insert: (OsString, FileTree),
) -> ScaffoldResult<()> {
    let v: Vec<OsString> = folder_path.iter().map(|s| s.to_os_string()).collect();
    file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(folder_path.to_path_buf()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(folder_path.to_path_buf()))?
        .insert(file_tree_to_insert.0, file_tree_to_insert.1);
    Ok(())
}

pub fn find_files_by_name(file_tree: &FileTree, file_name: &Path) -> BTreeMap<PathBuf, String> {
    find_files(file_tree, &|file_path, _file_contents| {
        file_name.file_name().eq(&file_path.file_name())
    })
}

pub fn find_files<F: Fn(&PathBuf, &String) -> bool>(
    file_tree: &FileTree,
    find_by_path_and_contents: &F,
) -> BTreeMap<PathBuf, String> {
    find_map_files(file_tree, &|file_name, file_contents| {
        if find_by_path_and_contents(file_name, file_contents) {
            Some(file_contents.clone())
        } else {
            None
        }
    })
}

pub fn find_map_rust_files<T, F: Fn(&PathBuf, &syn::File) -> Option<T>>(
    file_tree: &FileTree,
    find_fn: &F,
) -> BTreeMap<PathBuf, T> {
    find_map_files(file_tree, &|file_path, file_contents| {
        if let Some(extension) = file_path.extension() {
            if extension == "rs" {
                let result: Result<syn::File, _> = syn::parse_str(file_contents.as_str());

                if let Ok(file) = result {
                    if let Some(t) = find_fn(file_path, &file) {
                        return Some(t);
                    }
                }
            }
        }

        None
    })
}

pub fn find_map_files<T, F: Fn(&PathBuf, &String) -> Option<T>>(
    file_tree: &FileTree,
    find_by_path_and_contents: &F,
) -> BTreeMap<PathBuf, T> {
    find_map_files_rec(file_tree, find_by_path_and_contents, &PathBuf::new())
}

fn find_map_files_rec<T, F: Fn(&PathBuf, &String) -> Option<T>>(
    file_tree: &FileTree,
    find_by_path_and_contents: &F,
    current_path: &Path,
) -> BTreeMap<PathBuf, T> {
    let mut found_files: BTreeMap<PathBuf, T> = BTreeMap::new();

    match file_tree {
        FileTree::File(_) => {}
        FileTree::Directory(directory_contents) => {
            for (file_name, child_file_tree) in directory_contents {
                let child_path = current_path.join(file_name);

                if let FileTree::File(contents) = child_file_tree {
                    if let Some(t) = find_by_path_and_contents(&child_path, contents) {
                        found_files.insert(child_path, t);
                    }
                } else {
                    let sub_paths =
                        find_map_files_rec(child_file_tree, find_by_path_and_contents, &child_path);
                    for (grandchild_path, contents) in sub_paths {
                        found_files.insert(grandchild_path, contents);
                    }
                }
            }
        }
    }

    found_files
}

pub fn map_rust_files<F: Fn(PathBuf, syn::File) -> ScaffoldResult<syn::File> + Copy>(
    file_tree: &mut FileTree,
    map_fn: F,
) -> ScaffoldResult<()> {
    map_all_files(file_tree, |file_path, contents| {
        if let Some(extension) = file_path.extension() {
            if extension == "rs" {
                let original_file: syn::File = syn::parse_str(&contents)
                    .map_err(|e| ScaffoldError::MalformedFile(file_path.clone(), e.to_string()))?;
                let new_file = map_fn(file_path, original_file.clone())?;
                // Only reformat the file via unparse_pretty if the contents of the newly modified
                // file are different from the original
                if new_file != original_file {
                    return Ok(unparse_pretty(&new_file));
                }
            }
        }
        Ok(contents)
    })
}

pub fn flatten_file_tree(file_tree: &FileTree) -> BTreeMap<PathBuf, Option<String>> {
    walk_file_tree_rec(file_tree, &PathBuf::new())
}

pub fn unflatten_file_tree(
    flattened_tree: &BTreeMap<PathBuf, Option<String>>,
) -> ScaffoldResult<FileTree> {
    let mut file_tree: FileTree = FileTree::Directory(BTreeMap::new());

    for (path, maybe_contents) in flattened_tree.iter() {
        if let Some(contents) = maybe_contents {
            let mut folder_path = path.clone();
            folder_path.pop();

            create_dir_all(&mut file_tree, &folder_path)?;

            let v: Vec<OsString> = folder_path
                .clone()
                .iter()
                .map(|s| s.to_os_string())
                .collect();
            file_tree
                .path_mut(&mut v.iter())
                .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
                .dir_content_mut()
                .ok_or(ScaffoldError::PathNotFound(folder_path.clone()))?
                .insert(path.file_name().unwrap().to_os_string(), file!(contents));
        } else {
            create_dir_all(&mut file_tree, path)?;
        }
    }

    Ok(file_tree)
}

pub fn map_all_files<F: Fn(PathBuf, String) -> ScaffoldResult<String> + Copy>(
    file_tree: &mut FileTree,
    map_fn: F,
) -> ScaffoldResult<()> {
    map_all_files_rec(file_tree, PathBuf::new(), map_fn)?;
    Ok(())
}

fn map_all_files_rec<F: Fn(PathBuf, String) -> ScaffoldResult<String> + Copy>(
    file_tree: &mut FileTree,
    current_path: PathBuf,
    map_fn: F,
) -> ScaffoldResult<()> {
    if let Some(dir) = file_tree.dir_content_mut() {
        for (key, mut tree) in dir.clone().into_iter() {
            let child_path = current_path.join(&key);
            match &tree {
                FileTree::Directory(_) => {
                    map_all_files_rec(&mut tree, child_path, map_fn)?;
                }
                FileTree::File(file_contents) => {
                    *tree
                        .file_content_mut()
                        .context("Failed to get mutable reference of file tree")? =
                        map_fn(child_path, file_contents.to_owned())?;
                }
            }
            dir.insert(key, tree);
        }
    }

    Ok(())
}

pub fn create_dir_all(file_tree: &mut FileTree, path: &Path) -> ScaffoldResult<()> {
    let mut current_path = PathBuf::new();

    for c in path.components() {
        let v: Vec<OsString> = current_path
            .clone()
            .iter()
            .map(|s| s.to_os_string())
            .collect();
        if let Some(contents) = file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(current_path.clone()))?
            .dir_content_mut()
        {
            let component_key = c.as_os_str().to_os_string();
            contents
                .entry(component_key)
                .or_insert_with(|| FileTree::Directory(BTreeMap::new()));
        } else {
            return Err(ScaffoldError::InvalidPath(
                path.to_path_buf(),
                String::from("given path is a file, and we expected it to be a directory"),
            ));
        }

        current_path.push(c);
    }

    Ok(())
}

pub fn template_dirs_to_file_tree(
    ui_framework_template_dir: &Dir<'_>,
    generic_template_dir: &Dir<'_>,
) -> ScaffoldResult<FileTree> {
    let mut flattened = walk_dir(ui_framework_template_dir);
    flattened.extend(walk_dir(generic_template_dir));
    unflatten_file_tree(&flattened)
}

fn walk_dir(dir: &Dir<'_>) -> BTreeMap<PathBuf, Option<String>> {
    let mut contents: BTreeMap<PathBuf, Option<String>> = BTreeMap::new();

    for f in dir.files() {
        if let Some(s) = f.contents_utf8() {
            contents.insert(f.path().to_path_buf(), Some(s.to_string()));
        }
    }
    for d in dir.dirs() {
        contents.insert(d.path().to_path_buf(), None);
        contents.extend(walk_dir(d));
    }

    contents
}

fn walk_file_tree_rec(
    file_tree: &FileTree,
    current_path: &Path,
) -> BTreeMap<PathBuf, Option<String>> {
    let mut found_files: BTreeMap<PathBuf, Option<String>> = BTreeMap::new();

    match file_tree {
        FileTree::File(_) => {}
        FileTree::Directory(directory_contents) => {
            for (file_name, child_file_tree) in directory_contents {
                let child_path = current_path.join(file_name);

                if let FileTree::File(contents) = child_file_tree {
                    found_files.insert(child_path, Some(contents.clone()));
                } else {
                    found_files.insert(child_path.clone(), None);
                    let sub_paths = walk_file_tree_rec(child_file_tree, &child_path);
                    for (grandchild_path, contents) in sub_paths {
                        found_files.insert(grandchild_path, contents);
                    }
                }
            }
        }
    }

    found_files
}

pub fn build_file_tree(file_tree: FileTree, path: impl Into<PathBuf>) -> Result<(), ScaffoldError> {
    let mergeable_tree = MergeableFileSystemTree::from(file_tree);
    mergeable_tree.build(&path.into())?;
    Ok(())
}
