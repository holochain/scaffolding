use holochain_scaffolding_utils::*;

use workdir::generate_web_happ_workdir;

mod workdir;

pub fn web_app_skeleton(app_name: &String) -> FileTree {
    dir! {
        "workdir" => generate_web_happ_workdir(app_name, &String::from("ui"))
    }
}

pub enum GenerateDnaError {}
pub fn generate_dna(web_app_dir_tree: FileTree) -> Result<FileTree, GenerateDnaError> {
    Ok(web_app_dir_tree)
}

