use std::path::PathBuf;

use build_fs_tree::{dir, file};
use holochain_scaffolding_utils::*;

fn fixture_path() -> PathBuf {
    std::env::current_dir().unwrap().join("tests/fixture")
}

fn fixture_tree() -> FileTree {
    dir! {
        "example.sh" => file!(""),
        "dir" => dir! {
            "some_script.sh" => file!("Hello!")
        }
    }
}

#[test]
fn test_load_dir_into_memory() {
    let result = load_directory_into_memory(&fixture_path()).unwrap();

    let expected = fixture_tree();

    assert_eq!(result, expected);
}

#[test]
fn test_find_files() {
    let file_tree = fixture_tree();

    let paths = find_files_by_name(&file_tree, &PathBuf::new().join("some_script.sh"));

    assert_eq!(
        paths
            .get(&PathBuf::new().join("dir").join("some_script.sh"))
            .unwrap()
            .clone(),
        String::from("Hello!")
    );
}
