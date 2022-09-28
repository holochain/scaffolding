use std::path::PathBuf;

use vfs::{MemoryFS, VfsResult, PhysicalFS, FileSystem, VfsPath};


pub fn load_dir_into_memory(path: PathBuf) -> VfsResult<MemoryFS> {
    let memory_fs = MemoryFS::new();

    let physical = PhysicalFS::new(path);

    for file_result in VfsPath::new(physical).walk_dir()? {
        let file = file_result?;

        match file.is_dir()? {
            true => memory_fs.create_dir(file.as_str())?,
            false => {
                memory_fs.create_file(file.as_str())?.write(file.read_to_string()?.as_bytes())?;
            }
        }
    }


    Ok(memory_fs)
}

pub fn write_to_disk(path: PathBuf, filesystem: MemoryFS) -> VfsResult<()> {

    

    Ok(())
}