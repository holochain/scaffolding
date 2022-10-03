use std::collections::BTreeMap;
use std::{fs, io, path::PathBuf};

use build_fs_tree::FileSystemTree;
pub use build_fs_tree::{dir, file, Build};

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