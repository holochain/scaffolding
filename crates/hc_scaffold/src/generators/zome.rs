use holochain_scaffolding_utils::FileTree;

use crate::error::ScaffoldResult;

use super::{app::utils::get_or_choose_app_manifest, dna::utils::get_or_choose_dna_manifest};

pub fn scaffold_integrity_zome(
    app_file_tree: FileTree,
    app_name: Option<String>,
    dna_name: Option<String>,
    zome_name: String,
) -> ScaffoldResult<FileTree> {
    let app_manifest = get_or_choose_app_manifest(&app_file_tree, app_name)?;
    let (dna_manifest_path, dna_manifest) =
        get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna_name)?;

        

    Ok(app_file_tree)
}
