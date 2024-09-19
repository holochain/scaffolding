use std::str::FromStr;

use colored::Colorize;
use convert_case::Case;
use structopt::StructOpt;

use crate::{
    file_tree::{build_file_tree, load_directory_into_memory},
    scaffold::{
        collection::{scaffold_collection, CollectionType},
        dna::DnaFileTree,
        entry_type::definitions::EntryTypeReference,
        web_app::template_type::TemplateType,
        zome::ZomeFileTree,
    },
    templates::ScaffoldedTemplate,
    utils::{check_case, input_with_case, run_cargo_fmt_if_available},
};

#[derive(Debug, StructOpt)]
/// Scaffold a collection of entries in an existing zome
pub struct Collection {
    #[structopt(long)]
    /// Name of the dna in which you want to scaffold the zome
    pub dna: Option<String>,

    #[structopt(long)]
    /// Name of the integrity zome in which you want to scaffold the link type
    pub zome: Option<String>,

    /// Collection type: "global" or "by-author"
    pub collection_type: Option<CollectionType>,

    /// Collection name, just to differentiate it from other collections
    pub collection_name: Option<String>,

    #[structopt(parse(try_from_str = EntryTypeReference::from_str))]
    /// Entry type that is going to be added to the collection
    pub entry_type: Option<EntryTypeReference>,

    #[structopt(long)]
    /// Skips UI generation for this collection.
    pub no_ui: bool,
}

impl Collection {
    pub fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let file_tree = load_directory_into_memory(&current_dir)?;

        let dna_file_tree = DnaFileTree::get_or_choose(file_tree, self.dna.as_deref())?;
        let zome_file_tree =
            ZomeFileTree::get_or_choose_integrity(dna_file_tree, self.zome.as_deref())?;

        let name = match self.collection_name {
            Some(n) => {
                check_case(&n, "collection name", Case::Snake)?;
                n
            }
            None => input_with_case(
                "Collection name (snake_case, eg. \"all_posts\"):",
                Case::Snake,
            )?,
        };

        let ScaffoldedTemplate {
            file_tree,
            next_instructions,
        } = scaffold_collection(
            zome_file_tree,
            &template_type.file_tree()?,
            &name,
            self.collection_type,
            self.entry_type,
            self.no_ui,
        )?;

        build_file_tree(file_tree, ".")?;

        if let Err(e) = run_cargo_fmt_if_available() {
            println!(
                "{}: {}",
                "rustfmt exec failed: ".yellow(),
                e.to_string().yellow()
            );
        }

        println!("\nCollection {} scaffolded!", name.italic());

        if let Some(i) = next_instructions {
            println!("\n{}", i);
        }

        Ok(())
    }
}
