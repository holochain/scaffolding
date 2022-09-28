use vfs::{FileSystem, MemoryFS};

pub fn generate_web_app_skeleton() -> MemoryFS {
    let memfs = MemoryFS::new();

    memfs.create_file("hey").unwrap().write("buf".as_bytes()).unwrap();

    memfs
}