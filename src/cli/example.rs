use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::{Build, MergeableFileSystemTree};
use colored::Colorize;
use structopt::StructOpt;
use tokio::fs;

use crate::{
    error::ScaffoldError,
    file_tree::build_file_tree,
    scaffold::{
        app::{git::setup_git_environment, nix::setup_nix_developer_environment, AppFileTree},
        collection::{scaffold_collection, CollectionType},
        config::ScaffoldConfig,
        dna::{scaffold_dna, DnaFileTree},
        entry_type::{
            crud::Crud,
            definitions::{
                Cardinality, EntryTypeReference, FieldDefinition, FieldType, Referenceable,
            },
            scaffold_entry_type,
        },
        example::ExampleType,
        web_app::{scaffold_web_app, template_type::TemplateType},
        zome::{
            scaffold_coordinator_zome_in_path, scaffold_integrity_zome_with_path, ZomeFileTree,
        },
    },
    templates::{example::scaffold_example, ScaffoldedTemplate},
    utils::run_cargo_fmt_if_available,
};

#[derive(Debug, StructOpt)]
/// Scaffold an example hApp
pub struct Example {
    /// Name of the example to scaffold. One of ['hello-world', 'forum'].
    pub example: Option<ExampleType>,

    #[structopt(long)]
    /// Whether to setup the holonix development environment for the example hApp
    pub setup_nix: Option<bool>,
}

impl Example {
    pub async fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        let command_root_dir = std::env::current_dir()?;
        let template_file_tree = template_type.file_tree()?;
        let template_name = template_type.name();
        let is_vanilla_template = matches!(template_type, TemplateType::Vanilla);

        let example = match self.example {
            Some(e) => e,
            None => {
                if is_vanilla_template {
                    println!("Scaffolding the {} example project", "hello-world".italic());
                    ExampleType::HelloWorld
                } else {
                    ExampleType::Forum
                }
            }
        };
        let example_name = example.to_string();

        let app_dir = command_root_dir.join(&example_name);
        if app_dir.as_path().exists() {
            return Err(ScaffoldError::FolderAlreadyExists(app_dir.clone()))?;
        }

        // Ensure the correct template is used for each example
        if matches!(example, ExampleType::HelloWorld) && !is_vanilla_template
            || matches!(example, ExampleType::Forum) && is_vanilla_template
        {
            return Err(ScaffoldError::InvalidArguments(format!(
                "{} example cannot be used with the {} template.",
                example.to_string().italic(),
                &template_name.italic(),
            ))
            .into());
        }

        // Match on example types
        let file_tree = match example {
            ExampleType::HelloWorld => {
                // scaffold web-app
                let ScaffoldedTemplate { file_tree, .. } = scaffold_web_app(
                    &example_name,
                    Some("A simple 'hello world' application."),
                    false,
                    &template_file_tree,
                )?;

                file_tree
            }
            ExampleType::Forum => {
                // scaffold web-app
                let ScaffoldedTemplate { file_tree, .. } = scaffold_web_app(
                    &example_name,
                    Some("A simple 'forum' application."),
                    false,
                    &template_file_tree,
                )?;

                // scaffold dna hello_world
                let dna_name = "forum";

                let app_file_tree = AppFileTree::get_or_choose(file_tree, Some(&example_name))?;
                let ScaffoldedTemplate { file_tree, .. } =
                    scaffold_dna(app_file_tree, &template_file_tree, dna_name)?;

                // scaffold integrity zome posts
                let dna_file_tree = DnaFileTree::get_or_choose(file_tree, Some(dna_name))?;
                let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

                let integrity_zome_name = "posts_integrity";
                let integrity_zome_path = PathBuf::new()
                    .join("dnas")
                    .join(dna_name)
                    .join("zomes")
                    .join("integrity");
                let ScaffoldedTemplate { file_tree, .. } = scaffold_integrity_zome_with_path(
                    dna_file_tree,
                    &template_file_tree,
                    integrity_zome_name,
                    &integrity_zome_path,
                )?;

                let dna_file_tree =
                    DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

                let coordinator_zome_name = "posts";
                let coordinator_zome_path = PathBuf::new()
                    .join("dnas")
                    .join(dna_name)
                    .join("zomes")
                    .join("coordinator");
                let ScaffoldedTemplate { file_tree, .. } = scaffold_coordinator_zome_in_path(
                    dna_file_tree,
                    &template_file_tree,
                    coordinator_zome_name,
                    Some(&vec![integrity_zome_name.to_owned()]),
                    &coordinator_zome_path,
                )?;

                // Scaffold the app here to enable ZomeFileTree::from_manifest(), which calls `cargo metadata`
                MergeableFileSystemTree::<OsString, String>::from(file_tree.clone())
                    .build(&app_dir)?;

                std::env::set_current_dir(&app_dir)?;

                let dna_file_tree =
                    DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

                let zome_file_tree = ZomeFileTree::get_or_choose_integrity(
                    dna_file_tree,
                    Some(integrity_zome_name),
                )?;

                let post_entry_type_name = "post";

                let ScaffoldedTemplate { file_tree, .. } = scaffold_entry_type(
                    zome_file_tree,
                    &template_file_tree,
                    "post",
                    Some(Crud {
                        update: true,
                        delete: true,
                    }),
                    Some(false),
                    Some(true),
                    Some(&vec![
                        FieldDefinition {
                            field_name: "title".to_string(),
                            field_type: FieldType::String,
                            widget: Some("TextField".to_string()),
                            cardinality: Cardinality::Single,
                            linked_from: None,
                        },
                        FieldDefinition {
                            field_name: "content".to_string(),
                            field_type: FieldType::String,
                            widget: Some("TextArea".to_string()),
                            cardinality: Cardinality::Single,
                            linked_from: None,
                        },
                    ]),
                    false,
                    false,
                )?;

                let dna_file_tree =
                    DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

                let zome_file_tree =
                    ZomeFileTree::get_or_choose_integrity(dna_file_tree, Some("posts_integrity"))?;

                let ScaffoldedTemplate { file_tree, .. } = scaffold_entry_type(
                    zome_file_tree,
                    &template_file_tree,
                    "comment",
                    Some(Crud {
                        update: false,
                        delete: true,
                    }),
                    Some(false),
                    Some(true),
                    Some(&vec![
                        FieldDefinition {
                            field_name: "comment".to_string(),
                            field_type: FieldType::String,
                            widget: Some("TextArea".to_string()),
                            cardinality: Cardinality::Single,
                            linked_from: None,
                        },
                        FieldDefinition {
                            field_name: "post_hash".to_string(),
                            field_type: FieldType::ActionHash,
                            widget: None,
                            cardinality: Cardinality::Single,
                            linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                                entry_type: post_entry_type_name.to_string(),
                                reference_entry_hash: false,
                            })),
                        },
                    ]),
                    false,
                    false,
                )?;

                let dna_file_tree =
                    DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

                let zome_file_tree = ZomeFileTree::get_or_choose_integrity(
                    dna_file_tree,
                    Some(integrity_zome_name),
                )?;

                let ScaffoldedTemplate { file_tree, .. } = scaffold_collection(
                    zome_file_tree,
                    &template_file_tree,
                    "all_posts",
                    Some(CollectionType::Global),
                    Some(EntryTypeReference {
                        entry_type: "post".to_string(),
                        reference_entry_hash: false,
                    }),
                    false,
                    false,
                )?;

                file_tree
            }
        };

        let ScaffoldedTemplate {
            mut file_tree,
            next_instructions,
        } = scaffold_example(file_tree, &template_file_tree, &example)?;

        ScaffoldConfig::write_to_package_json(&mut file_tree, template_type)?;

        build_file_tree(file_tree, &app_dir)?;

        // cargo fmt needs to be run inside the Rust project folder
        std::env::set_current_dir(&app_dir)?;

        if let Err(e) = run_cargo_fmt_if_available() {
            println!(
                "{}: {}",
                "rustfmt exec failed: ".yellow(),
                e.to_string().yellow()
            );
        }

        // set up nix
        if let Some(true) | None = self.setup_nix {
            if let Err(err) = setup_nix_developer_environment(&command_root_dir, &app_dir) {
                fs::remove_dir_all(&app_dir).await?;
                return Err(err)?;
            }
        };

        setup_git_environment(&app_dir)?;

        println!("\nExample {} scaffolded!\n", example.to_string().italic());

        if let Some(i) = next_instructions {
            println!("{i}");
        }

        Ok(())
    }
}
