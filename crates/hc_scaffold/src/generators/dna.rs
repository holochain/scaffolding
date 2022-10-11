pub fn scaffold_dna(app_file_tree: FileTree, dna_name: String) -> anyhow::Result<()> {
    let file_tree = MergeableFileSystemTree::<String, String>::from(dir! {
      app_name.clone() => web_app_skeleton(app_name, description)?
    });

    file_tree.build(&".".into())?;

    Ok(())
}
