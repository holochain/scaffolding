use colored::Colorize;
use convert_case::Case;
use structopt::StructOpt;

use crate::{
    file_tree::{build_file_tree, load_directory_into_memory, FileTree},
    scaffold::{
        dna::DnaFileTree,
        entry_type::{
            crud::{parse_crud, Crud},
            definitions::FieldDefinition,
            fields::parse_fields,
            scaffold_entry_type,
        },
        zome::ZomeFileTree,
    },
    templates::ScaffoldedTemplate,
    utils::{check_case, input_with_case},
};

#[derive(Debug, StructOpt)]
pub struct EntryType {
    #[structopt(long)]
    /// Name of the dna in which you want to scaffold the zome
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

    #[structopt(long, parse(try_from_str = parse_crud))]
    /// The Create, "Read", "Update", and "Delete" zome call functions that should be scaffolded for this entry type
    /// If "--reference-entry-hash" is "true", only "Create" and "Read" will be scaffolded
    pub crud: Option<Crud>,

    #[structopt(long)]
    /// Whether to create a link from the original entry to each update action
    /// Only applies if update is selected in the "crud" argument
    pub link_from_original_to_each_update: Option<bool>,

    #[structopt(long, value_delimiter = ",", parse(try_from_str = parse_fields))]
    /// The fields that the entry type struct should contain
    /// Grammar: <FIELD_NAME>:<FIELD_TYPE>:<WIDGET>:<LINKED_FROM> , (widget and linked_from are optional)
    /// Eg. "title:String:TextField" , "posts_hashes:Vec\<ActionHash\>::Post"
    pub fields: Option<Vec<FieldDefinition>>,

    #[structopt(long)]
    /// Skips UI generation for this entry-type, overriding any specified widgets in the --fields option.
    pub no_ui: bool,
}

impl EntryType {
    pub fn run(self, template_file_tree: FileTree) -> anyhow::Result<()> {
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

        if self.no_ui {
            let warning_text = r#"
WARNING: Opting out of UI generation for this entry-type but not for other entry-types, link-types or collections associated with it
may result in potential UI inconsistencies. Specifically, UI elements intended for associated entry-types, link-types or collections could 
inadvertently reference or expect elements from the skipped entry type."#
                .yellow();
            println!("{warning_text}");
        }

        let ScaffoldedTemplate {
            file_tree,
            next_instructions,
        } = scaffold_entry_type(
            zome_file_tree,
            &template_file_tree,
            &name,
            self.crud,
            self.reference_entry_hash,
            self.link_from_original_to_each_update,
            self.fields.as_ref(),
            self.no_ui,
        )?;

        build_file_tree(file_tree, ".")?;

        println!("\nEntry type {} scaffolded!", name.italic());

        if let Some(i) = next_instructions {
            println!("\n{}", i);
        } else {
            println!(
                r#"
Add new collections for that entry type with:

  hc scaffold collection"#,
            );
        }

        Ok(())
    }
}
