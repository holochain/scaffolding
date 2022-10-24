use build_fs_tree::{dir, file, FileSystemTree};
use ignore::{Walk, WalkBuilder};
use prettyplease::unparse;
use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::path::Iter;
use std::{fs, io, path::PathBuf};

use crate::error::{ScaffoldError, ScaffoldResult};

pub type FileTree = FileSystemTree<OsString, String>;

// Loads the directory tree in the given path into memory recursively
pub fn load_directory_into_memory(path: &PathBuf) -> ScaffoldResult<FileTree> {
    let mut file_tree: FileTree = dir! {};

    for result in WalkBuilder::new(&path).hidden(false).build() {
        let dir_entry = result?
            .path()
            .to_path_buf()
            .into_iter()
            .skip(path.components().count())
            .collect();

        if fs::metadata(&path.join(&dir_entry))?.is_dir() {
            create_dir_all(&mut file_tree, &dir_entry)?;
        } else {
            let contents = fs::read_to_string(&path.join(&dir_entry))?;

            let mut folder_path = dir_entry.clone();
            folder_path.pop();

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
                .insert(
                    dir_entry.file_name().unwrap().to_os_string(),
                    file!(contents),
                );
        }
    }

    Ok(file_tree)
}

pub fn find_files_by_name(file_tree: &FileTree, file_name: &PathBuf) -> BTreeMap<PathBuf, String> {
    find_files(file_tree, &|file_path, _file_contents| {
        file_name.file_name().eq(&file_path.file_name())
    })
}

pub fn find_files<F: Fn(&PathBuf, &String) -> bool>(
    file_tree: &FileTree,
    find_by_path_and_contents: &F,
) -> BTreeMap<PathBuf, String> {
    find_map_files(
        file_tree,
        &|file_name, file_contents| match find_by_path_and_contents(file_name, file_contents) {
            true => Some(file_contents.clone()),
            false => None,
        },
    )
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
    current_path: &PathBuf,
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

pub fn map_rust_files<F: Fn(PathBuf, syn::File) -> ScaffoldResult<syn::File> + Clone>(
    file_tree: &mut FileTree,
    map_fn: F,
) -> ScaffoldResult<()> {
    map_all_files(file_tree, |file_path, s| {
        if let Some(extension) = file_path.extension() {
            if extension == "rs" {
                let rust_file: syn::File = syn::parse_str(s.as_str()).map_err(|e| {
                    ScaffoldError::MalformedFile(file_path.clone(), format!("{}", e))
                })?;

                let new_file = map_fn(file_path, rust_file)?;

                return Ok(unparse(&new_file));
            }
        }

        Ok(s)
    })
}

pub fn map_all_files<F: Fn(PathBuf, String) -> ScaffoldResult<String> + Clone>(
    file_tree: &mut FileTree,
    map_fn: F,
) -> ScaffoldResult<()> {
    map_all_files_rec(file_tree, PathBuf::new(), map_fn)
}

fn map_all_files_rec<F: Fn(PathBuf, String) -> ScaffoldResult<String> + Clone>(
    file_tree: &mut FileTree,
    current_path: PathBuf,
    map_fn: F,
) -> ScaffoldResult<()> {
    if let Some(c) = file_tree.dir_content_mut() {
        for (key, mut tree) in c.clone().into_iter() {
            let child_path = current_path.join(&key);
            match tree.clone() {
                FileTree::Directory(_dir_contents) => {
                    map_all_files_rec(&mut tree, child_path, map_fn.clone())?;
                }
                FileTree::File(file_contents) => {
                    *tree.file_content_mut().unwrap() = map_fn(child_path, file_contents)?;
                }
            }

            c.insert(key.clone(), tree.clone());
        }
    }

    Ok(())
}

pub fn create_dir_all(file_tree: &mut FileTree, path: &PathBuf) -> ScaffoldResult<()> {
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
            if !contents.contains_key(&component_key) {
                contents.insert(component_key, FileTree::Directory(BTreeMap::new()));
            }
        } else {
            return Err(ScaffoldError::InvalidPath(
                path.clone(),
                String::from("given path is a file, and we expected it to be a directory"),
            ));
        }

        current_path.push(c);
    }

    Ok(())
}
