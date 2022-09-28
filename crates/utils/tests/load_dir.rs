use holochain_scaffolding_utils::load_dir_into_memory;

use vfs::{VfsResult, VfsPath};

#[test]
fn test_load_dir_into_memory() {
    let result = load_dir_into_memory(std::env::current_dir().unwrap().join("tests/fixture")).unwrap();

    let root_path = VfsPath::new(result);

    let mut directories = root_path.walk_dir().unwrap().collect::<VfsResult<Vec<_>>>().unwrap();
    directories.sort_by_key(|path| path.as_str().to_string());

    let expected = vec!["dir", "dir/some_script.sh", "example.sh"]
        .iter()
        .map(|path| root_path.join(path))
        .collect::<VfsResult<Vec<_>>>().unwrap();

    assert_eq!(directories, expected);
}
