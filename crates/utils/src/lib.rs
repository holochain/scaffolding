use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fmt::Display;
use std::{fs, io, path::PathBuf};

use build_fs_tree::FileSystemTree;

pub type FileTree = FileSystemTree<String, String>;

// Loads the directory tree in the given path into memory recursively
pub fn load_directory_into_memory(path: &PathBuf) -> io::Result<FileTree> {
    let mut dir_contents: BTreeMap<String, FileTree> = BTreeMap::new();

    for maybe_entry in fs::read_dir(path)? {
        let entry = maybe_entry?;

        if entry.file_type()?.is_dir() {
            let subdirectory = load_directory_into_memory(&path.join(entry.file_name()))?;
            dir_contents.insert(
                entry.file_name().to_str().unwrap().to_string(),
                subdirectory,
            );
        } else {
            let contents = fs::read_to_string(path.join(entry.file_name()))?;

            dir_contents.insert(
                entry.file_name().to_str().unwrap().to_string(),
                FileSystemTree::File(contents),
            );
        }
    }

    Ok(FileSystemTree::Directory(dir_contents))
}

#[derive(Debug)]
pub enum OverrideFileContentsError {
    FileNotFound,
}

impl Display for OverrideFileContentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
/// Overrides the contents of the file found in the given `file_path` with the given `new_contents`
pub fn override_file_contents(
    file_tree: &mut FileTree,
    file_path: &PathBuf,
    new_contents: &String,
) -> Result<(), OverrideFileContentsError> {
    let file_name = file_path
        .file_name()
        .ok_or(OverrideFileContentsError::FileNotFound)?
        .to_str()
        .ok_or(OverrideFileContentsError::FileNotFound)?
        .to_string();
    let mut current_tree = file_tree;

    let maybe_parent = file_path.parent();

    if let Some(parent) = maybe_parent {
        for component in parent.components() {
            let directory_folder = component
                .as_os_str()
                .to_str()
                .ok_or(OverrideFileContentsError::FileNotFound)?
                .to_string();

            match current_tree {
                FileSystemTree::Directory(directory_contents) => {
                    current_tree = directory_contents
                        .get_mut(&directory_folder)
                        .ok_or(OverrideFileContentsError::FileNotFound)?;
                }
                FileSystemTree::File(_) => {
                    return Err(OverrideFileContentsError::FileNotFound);
                }
            }
        }
    }

    match current_tree {
        FileSystemTree::Directory(directory_contents) => {
            directory_contents.insert(file_name, FileTree::File(new_contents.clone()));
        }
        FileSystemTree::File(_) => {
            return Err(OverrideFileContentsError::FileNotFound);
        }
    }
    let dir_content = current_tree
        .dir_content()
        .ok_or(OverrideFileContentsError::FileNotFound)?;

    Ok(())
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
    find_files_rec(file_tree, find_by_path_and_contents, &PathBuf::new())
}

fn find_files_rec<F: Fn(&PathBuf, &String) -> bool>(
    file_tree: &FileTree,
    find_by_path_and_contents: &F,
    current_path: &PathBuf,
) -> BTreeMap<PathBuf, String> {
    let mut found_files: BTreeMap<PathBuf, String> = BTreeMap::new();

    match file_tree {
        FileTree::File(_) => {}
        FileTree::Directory(directory_contents) => {
            for (file_name, child_file_tree) in directory_contents {
                let child_path = current_path.join(file_name);

                if let FileTree::File(contents) = child_file_tree {
                    if find_by_path_and_contents(&child_path, contents) {
                        found_files.insert(child_path, contents.clone());
                    }
                } else {
                    let sub_paths =
                        find_files_rec(child_file_tree, find_by_path_and_contents, &child_path);
                    for (grandchild_path, contents) in sub_paths {
                        found_files.insert(grandchild_path, contents);
                    }
                }
            }
        }
    }

    found_files
}
