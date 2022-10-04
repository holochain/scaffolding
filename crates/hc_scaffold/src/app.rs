use holochain_scaffolding_utils::{FileTree};
use build_fs_tree::{dir, MergeableFileSystemTree, Build};

use crate::error::ScaffoldResult;

use self::default_nix::default_nix;

mod default_nix;


fn web_app(app_name: String) -> ScaffoldResult<FileTree> {
    Ok(dir! {
      "default.nix" => default_nix("main".into())
    })
}

pub fn scaffold_web_app(app_name: String) -> anyhow::Result<()> {
    let file_tree = MergeableFileSystemTree::<String, String>::from(dir! {
      app_name.clone() => web_app(app_name)?
    });

    file_tree.build(&".".into())?;

    Ok(())
}
