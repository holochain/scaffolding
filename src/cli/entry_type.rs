use std::str::FromStr;

use colored::Colorize;
use convert_case::Case;
use structopt::StructOpt;

use crate::{
    file_tree::{build_file_tree, load_directory_into_memory},
    scaffold::{
        dna::DnaFileTree,
        entry_type::{crud::Crud, definitions::FieldDefinition, scaffold_entry_type},
        web_app::template_type::TemplateType,
        zome::ZomeFileTree,
    },
    templates::ScaffoldedTemplate,
    utils::{check_case, input_with_case, run_cargo_fmt_if_available},
};

#[derive(Debug, StructOpt)]
/// Scaffold an entry type and CRUD functions into an existing zome
pub struct EntryType {
    #[structopt(long)]
    /// Name of the dna in which you want to scaffold the entry type
    pub dna: Option<String>,

    #[structopt(long)]
    /// Name of the integrity zome in which you want to scaffold the entry definition
    pub zome: Option<String>,

    /// Name of the entry type being scaffolded
    pub name: Option<String>,

    #[structopt(long)]
    /// Whether this entry type should be referenced with its "EntryHash" or its "ActionHash"
    /// If referred to by "EntryHash", the entries can't be updated or deleted
    pub reference_entry_hash: Option<bool>,

    #[structopt(long, parse(try_from_str = Crud::from_str))]
    /// The Create, "Read", "Update", and "Delete" zome call functions that should be scaffolded for this entry type
    /// If "--reference-entry-hash" is "true", only "Create" and "Read" will be scaffolded
    pub crud: Option<Crud>,

    #[structopt(long)]
    /// Whether to create a link from the original entry to each update action
    /// Only applies if update is selected in the "crud" argument
    pub link_from_original_to_each_update: Option<bool>,

    #[structopt(long, value_delimiter = ",", parse(try_from_str = FieldDefinition::from_str))]
    /// The fields that the entry type struct should contain
    /// Syntax: <FIELD_NAME>:<FIELD_TYPE>:<WIDGET>:<LINKED_FROM> , (widget and linked_from are optional)
    /// Eg. "title:String:TextField" , "posts_hashes:Vec\<ActionHash\>::Post"
    pub fields: Option<Vec<FieldDefinition>>,

    #[structopt(long)]
    /// Skips UI generation for this entry-type, overriding any specified widgets in the --fields option.
    pub no_ui: bool,

    #[structopt(long)]
    /// Skips test generation for this entry-type
    pub no_spec: bool,
}

impl EntryType {
    pub fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let file_tree = load_directory_into_memory(&current_dir)?;
        let name = match self.name {
            Some(n) => {
                check_case(&n, "entry type name", Case::Snake)?;
                n
            }
            None => input_with_case("Entry type name (snake_case):", Case::Snake)?,
        };

        let dna_file_tree = DnaFileTree::get_or_choose(file_tree, self.dna.as_deref())?;
        let zome_file_tree =
            ZomeFileTree::get_or_choose_integrity(dna_file_tree, self.zome.as_deref())?;

        let ScaffoldedTemplate {
            file_tree,
            next_instructions,
        } = scaffold_entry_type(
            zome_file_tree,
            &template_type.file_tree()?,
            &name,
            self.crud,
            self.reference_entry_hash,
            self.link_from_original_to_each_update,
            self.fields.as_ref(),
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

        println!("\nEntry type {} scaffolded!", name.italic());

        if let Some(i) = next_instructions {
            println!("\n{}", i);
        } else {
            println!(
                r#"
Add new collections for that entry type with:

  hc scaffold collection
                "#,
            );
        }

        Ok(())
    }
}
