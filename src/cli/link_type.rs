use std::str::FromStr;

use colored::Colorize;
use structopt::StructOpt;

use crate::{
    file_tree::{build_file_tree, load_directory_into_memory},
    scaffold::{
        dna::DnaFileTree, entry_type::definitions::Referenceable, link_type::scaffold_link_type,
        web_app::template_type::TemplateType, zome::ZomeFileTree,
    },
    templates::ScaffoldedTemplate,
    utils::run_cargo_fmt_if_available,
};

#[derive(Debug, StructOpt)]
/// Scaffold a link type and its appropriate zome functions into an existing zome
pub struct LinkType {
    #[structopt(long)]
    /// Name of the dna in which you want to scaffold the zome
    pub dna: Option<String>,

    #[structopt(long)]
    /// Name of the integrity zome in which you want to scaffold the link type
    pub zome: Option<String>,

    #[structopt(parse(try_from_str = Referenceable::from_str))]
    /// Entry type (or agent role) used as the base for the links
    pub from_referenceable: Option<Referenceable>,

    #[structopt(parse(try_from_str = Referenceable::from_str))]
    /// Entry type (or agent role) used as the target for the links
    pub to_referenceable: Option<Referenceable>,

    #[structopt(long)]
    /// Whether to create the inverse link, from the "--to-referenceable" entry type to the "--from-referenceable" entry type
    pub bidirectional: Option<bool>,

    #[structopt(long)]
    /// Whether this link type can be deleted
    pub delete: Option<bool>,

    #[structopt(long)]
    /// Skips UI generation for this link-type.
    pub no_ui: bool,
    #[structopt(long)]

    /// Skips test generation for this link-type.
    pub no_spec: bool,
}

impl LinkType {
    pub fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let file_tree = load_directory_into_memory(&current_dir)?;

        let dna_file_tree = DnaFileTree::get_or_choose(file_tree, self.dna.as_deref())?;
        let zome_file_tree =
            ZomeFileTree::get_or_choose_integrity(dna_file_tree, self.zome.as_deref())?;

        let ScaffoldedTemplate {
            file_tree,
            next_instructions,
        } = scaffold_link_type(
            zome_file_tree,
            &template_type.file_tree()?,
            self.from_referenceable.as_ref(),
            self.to_referenceable.as_ref(),
            self.delete,
            self.bidirectional,
            self.no_ui,
            self.no_spec,
        )?;

        build_file_tree(file_tree, ".")?;

        if let Err(e) = run_cargo_fmt_if_available() {
            println!(
                "{}: {}",
                "rustfmt exec failed: ".yellow(),
                e.to_string().yellow()
            );
        }

        println!("\nLink type scaffolded!\n");
        if let Some(i) = next_instructions {
            println!("{}", i);
        }

        Ok(())
    }
}
