use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::{Build, MergeableFileSystemTree};
use colored::Colorize;
use convert_case::Case;
use structopt::StructOpt;

use crate::{
    file_tree::load_directory_into_memory,
    scaffold::{
        app::cargo::exec_metadata,
        dna::DnaFileTree,
        web_app::template_type::TemplateType,
        zome::{
            integrity_zome_name, scaffold_coordinator_zome, scaffold_integrity_zome,
            utils::{select_integrity_zomes, select_scaffold_zome_options},
        },
    },
    templates::ScaffoldedTemplate,
    utils::{check_case, input_with_case, run_cargo_fmt_if_available},
};

#[derive(Debug, StructOpt)]
/// Scaffold one or multiple zomes into an existing DNA
pub struct Zome {
    #[structopt(long)]
    /// Name of the dna in which you want to scaffold the zome
    pub dna: Option<String>,

    /// Name of the zome being scaffolded
    pub name: Option<String>,

    #[structopt(long, parse(from_os_str))]
    /// Scaffold an integrity zome at the given path
    pub integrity: Option<PathBuf>,

    #[structopt(long, parse(from_os_str))]
    /// Scaffold a coordinator zome at the given path
    pub coordinator: Option<PathBuf>,
}

impl Zome {
    pub fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let file_tree = load_directory_into_memory(&current_dir)?;
        let template_file_tree = template_type.file_tree()?;

        if let Some(n) = self.name.clone() {
            check_case(&n, "zome name", Case::Snake)?;
        }

        let (scaffold_integrity, scaffold_coordinator) = match (&self.integrity, &self.coordinator)
        {
            (None, None) => select_scaffold_zome_options()?,
            _ => (self.integrity.is_some(), self.coordinator.is_some()),
        };

        let name_prompt = if scaffold_integrity && scaffold_coordinator {
            "Enter coordinator zome name (snake_case):\n (The integrity zome will automatically be named '{name of coordinator zome}_integrity')\n"
        } else {
            "Enter zome name (snake_case):"
        };

        let name = match self.name {
            Some(n) => n,
            None => input_with_case(name_prompt, Case::Snake)?,
        };

        let mut dna_file_tree = DnaFileTree::get_or_choose(file_tree, self.dna.as_deref())?;
        let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

        let mut zome_next_instructions: (Option<String>, Option<String>) = Default::default();

        if scaffold_integrity {
            let integrity_zome_name = if scaffold_coordinator {
                integrity_zome_name(&name)
            } else {
                name.clone()
            };
            let ScaffoldedTemplate {
                file_tree,
                next_instructions,
            } = scaffold_integrity_zome(
                dna_file_tree,
                &template_file_tree,
                &integrity_zome_name,
                &self.integrity,
            )?;

            zome_next_instructions.0 = next_instructions;

            println!(
                "\nIntegrity zome {} scaffolded!",
                integrity_zome_name.italic(),
            );

            dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
        }

        if scaffold_coordinator {
            let dependencies = {
                if scaffold_integrity {
                    Some(vec![integrity_zome_name(&name)])
                } else {
                    let integrity_zomes = select_integrity_zomes(&dna_file_tree.dna_manifest, Some(
                      "Select integrity zome(s) this coordinator zome depends on (SPACE to select/unselect, ENTER to continue):"
                    ))?;
                    Some(integrity_zomes)
                }
            };
            let ScaffoldedTemplate {
                file_tree,
                next_instructions,
            } = scaffold_coordinator_zome(
                dna_file_tree,
                &template_file_tree,
                &name,
                dependencies.as_ref(),
                &self.coordinator,
            )?;
            zome_next_instructions.1 = next_instructions;

            println!("\nCoordinator zome {} scaffolded!", name.italic());

            dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
        }

        // TODO: implement scaffold_zome_template
        let file_tree =
            MergeableFileSystemTree::<OsString, String>::from(dna_file_tree.file_tree());

        // FIXME: avoid cloning
        let f = file_tree.clone();
        file_tree.build(&PathBuf::from("."))?;

        if let Err(e) = run_cargo_fmt_if_available() {
            println!(
                "{}: {}",
                "rustfmt exec failed: ".yellow(),
                e.to_string().yellow()
            );
        }

        // Execute cargo metadata to set up the cargo workspace in case this zome is the first crate
        exec_metadata(&f)?;

        match zome_next_instructions {
            (Some(integrity), Some(coordinator)) => {
                println!("\n{integrity}");
                println!("\n{coordinator}");
            }
            (None, Some(coordinator)) => println!("\n{coordinator}"),
            (Some(integrity), None) => println!("\n{integrity}"),
            _ => println!(
                r#"
Add new entry definitions to your zome with:

  hc scaffold entry-type
"#,
            ),
        }

        Ok(())
    }
}
