use holochain_scaffolding_utils::*;

//use workdir::generate_web_happ_workdir;

pub mod error;
pub mod generators;
pub mod cli;

use error::ScaffoldResult;
/* 
pub fn web_app_skeleton(
    app_name: String,
    app_description: Option<String>,
) -> ScaffoldResult<FileTree> {
    Ok(dir! {
        "workdir" => generate_web_happ_workdir(app_name, app_description, &String::from("ui"))?
    })
} */

pub enum GenerateDnaError {}
pub fn generate_dna(web_app_dir_tree: FileTree) -> Result<FileTree, GenerateDnaError> {
    Ok(web_app_dir_tree)
}
