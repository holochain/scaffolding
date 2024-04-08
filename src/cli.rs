use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{file_content, load_directory_into_memory, map_file, FileTree};
use crate::scaffold::app::cargo::exec_metadata;
use crate::scaffold::app::nix::setup_nix_developer_environment;
use crate::scaffold::app::AppFileTree;
use crate::scaffold::collection::{scaffold_collection, CollectionType};
use crate::scaffold::dna::{scaffold_dna, DnaFileTree};
use crate::scaffold::entry_type::crud::{parse_crud, Crud};
use crate::scaffold::entry_type::definitions::{
    parse_entry_type_reference, parse_referenceable, Cardinality, EntryTypeReference,
    FieldDefinition, FieldType, Referenceable,
};
use crate::scaffold::entry_type::{fields::parse_fields, scaffold_entry_type};
use crate::scaffold::example::{choose_example, Example};
use crate::scaffold::link_type::scaffold_link_type;
use crate::scaffold::web_app::scaffold_web_app;
use crate::scaffold::web_app::uis::{
    choose_ui_framework, guess_or_choose_framework, template_for_ui_framework, UiFramework,
};
use crate::scaffold::zome::utils::{select_integrity_zomes, select_scaffold_zome_options};
use crate::scaffold::zome::{
    integrity_zome_name, scaffold_coordinator_zome, scaffold_coordinator_zome_in_path,
    scaffold_integrity_zome, scaffold_integrity_zome_with_path, ZomeFileTree,
};
use crate::templates::example::scaffold_example;
use crate::templates::ScaffoldedTemplate;
use crate::utils::{
    check_case, check_no_whitespace, input_no_whitespace, input_with_case, input_yes_or_no,
};

use build_fs_tree::{dir, Build, MergeableFileSystemTree};
use convert_case::Case;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::{env, fs};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct HcScaffold {
    #[structopt(short, long)]
    /// The template to use for the scaffold command
    /// Can either be an option from the built-in templates: "vanilla", "vue", "lit", "svelte"
    /// Or a path to a custom template
    template: Option<String>,

    #[structopt(subcommand)]
    command: HcScaffoldCommand,
}

/// The list of subcommands for `hc scaffold`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffoldCommand {
    /// Scaffold a new, empty web app
    WebApp {
        /// Name of the app to scaffold
        name: Option<String>,

        /// Description of the app to scaffold
        description: Option<String>,

        #[structopt(long)]
        /// Whether to setup the holonix development environment for this web-app
        setup_nix: bool,

        #[structopt(long = "holo", hidden = true)]
        holo_enabled: bool,

        /// Whether to setup an initial DNA and it's zome(s) after the web app is scaffolded
        #[structopt(long = "fast-forward")]
        fast_forward: bool,
    },
    /// Manage custom templates
    Template(HcScaffoldTemplate),
    /// Scaffold a DNA into an existing app
    Dna {
        #[structopt(long)]
        /// Name of the app in which you want to scaffold the DNA
        app: Option<String>,

        /// Name of the DNA being scaffolded
        name: Option<String>,
    },
    /// Scaffold one or multiple zomes into an existing DNA
    Zome {
        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        /// Name of the zome being scaffolded
        name: Option<String>,

        #[structopt(long, parse(from_os_str))]
        /// Scaffold an integrity zome at the given path
        integrity: Option<PathBuf>,

        #[structopt(long, parse(from_os_str))]
        /// Scaffold a coordinator zome at the given path
        coordinator: Option<PathBuf>,
    },
    /// Scaffold an entry type and CRUD functions into an existing zome
    EntryType {
        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the entry definition
        zome: Option<String>,

        /// Name of the entry type being scaffolded
        name: Option<String>,

        #[structopt(long)]
        /// Whether this entry type should be refereced with its "EntryHash" or its "ActionHash"
        /// If referred to by "EntryHash", the entries can't be updated or deleted
        reference_entry_hash: Option<bool>,

        #[structopt(long, parse(try_from_str = parse_crud))]
        /// The Create, "Read", "Update", and "Delete" zome call functions that should be scaffolded for this entry type
        /// If "--reference-entry-hash" is "true", only "Create" and "Read" will be scaffolded
        crud: Option<Crud>,

        #[structopt(long)]
        /// Whether to create a link from the original entry to each update action
        /// Only applies if update is selected in the "crud" argument
        link_from_original_to_each_update: Option<bool>,

        #[structopt(long, value_delimiter = ",", parse(try_from_str = parse_fields))]
        /// The fields that the entry type struct should contain
        /// Grammar: <FIELD_NAME>:<FIELD_TYPE>:<WIDGET>:<LINKED_FROM> , (widget and linked_from are optional)
        /// Eg. "title:String:TextField" , "posts_hashes:Vec\<ActionHash\>::Post"
        fields: Option<Vec<FieldDefinition>>,

        #[structopt(long)]
        /// Skip ui generation, overriding any widgets specified with the --fields option
        no_ui: bool,
    },
    /// Scaffold a link type and its appropriate zome functions into an existing zome
    LinkType {
        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the link type
        zome: Option<String>,

        #[structopt(parse(try_from_str = parse_referenceable))]
        /// Entry type (or agent role) used as the base for the links
        from_referenceable: Option<Referenceable>,

        #[structopt(parse(try_from_str = parse_referenceable))]
        /// Entry type (or agent role) used as the target for the links
        to_referenceable: Option<Referenceable>,

        #[structopt(long)]
        /// Whether to create the inverse link, from the "--to-referenceable" entry type to the "--from-referenceable" entry type
        bidirectional: Option<bool>,

        #[structopt(long)]
        /// Whether this link type can be deleted
        delete: Option<bool>,

        #[structopt(long)]
        /// Skip ui generation
        no_ui: bool,
    },
    /// Scaffold a collection of entries in an existing zome
    Collection {
        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the link type
        zome: Option<String>,

        /// Collection type: "global" or "by-author"
        collection_type: Option<CollectionType>,

        /// Collection name, just to differentiate it from other collections
        collection_name: Option<String>,

        #[structopt(parse(try_from_str = parse_entry_type_reference))]
        /// Entry type that is going to be added to the collection
        entry_type: Option<EntryTypeReference>,

        #[structopt(long)]
        /// Skip ui generation
        no_ui: bool,
    },

    Example {
        /// Name of the example to scaffold. One of ['hello-world', 'forum'].
        example: Option<Example>,

        #[structopt(long = "holo", hidden = true)]
        holo_enabled: bool,
    },
}

impl HcScaffold {
    pub async fn run(self) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let template_config = if let Some(t) = &self.template {
            // Only read from config if the template is inbuilt and not a path
            if Path::new(t).exists() {
                None
            } else {
                get_template_config(&current_dir)?
            }
        } else {
            None
        };
        let template = match (&template_config, &self.template) {
            (Some(config), Some(template)) if &config.template != template => {
                return Err(anyhow::anyhow!(format!(
                "The value {} passed with `--template` does not match the template the web-app was scaffolded with: {}",
                template, config.template
            )))
            }
            (Some(config), _) => Some(&config.template),
            (_, t) => t.as_ref(),
        };

        let (template, template_file_tree) = match template {
            Some(template) => {
                let template_name_or_path;
                let file_tree = match template.as_str() {
                    "lit" | "svelte" | "vanilla" | "vue" => {
                        let ui_framework = UiFramework::from_str(template)?;
                        template_name_or_path = ui_framework.to_string();
                        template_for_ui_framework(&ui_framework)?
                    }
                    custom_template_path => {
                        template_name_or_path = custom_template_path.to_string();
                        let templates_dir = current_dir.join(PathBuf::from(custom_template_path));
                        load_directory_into_memory(&templates_dir)?
                    }
                };
                (template_name_or_path.to_owned(), file_tree)
            }
            None => {
                let ui_framework = match self.command {
                    HcScaffoldCommand::WebApp { .. } => choose_ui_framework()?,
                    _ => {
                        let file_tree = load_directory_into_memory(&current_dir)?;
                        guess_or_choose_framework(&file_tree)?
                    }
                };
                (
                    ui_framework.to_string(),
                    template_for_ui_framework(&ui_framework)?,
                )
            }
        };

        match self.command {
            HcScaffoldCommand::WebApp {
                name,
                description,
                setup_nix,
                holo_enabled,
                fast_forward,
            } => {
                let name = match name {
                    Some(n) => {
                        check_no_whitespace(&n, "app name")?;
                        n
                    }
                    None => input_no_whitespace("App name (no whitespaces):")?,
                };

                let app_folder = current_dir.join(&name);

                if app_folder.as_path().exists() {
                    return Err(ScaffoldError::FolderAlreadyExists(app_folder.clone()))?;
                }

                if file_content(&template_file_tree, &PathBuf::from("web-app/README.md.hbs"))
                    .is_err()
                {
                    return Err(ScaffoldError::MalformedTemplate(
                        "Template does not contain a README.md.hbs file in its \"web-app\" directory"
                            .to_string(),
                    ))?;
                }

                let setup_nix = if setup_nix {
                    setup_nix
                } else {
                    input_yes_or_no(
                        "Do you want to set up the holonix development environment for this project?", 
                        Some(true)
                    )?
                };

                let ScaffoldedTemplate {
                    file_tree,
                    next_instructions,
                } = scaffold_web_app(
                    name.clone(),
                    description,
                    !setup_nix,
                    &template_file_tree,
                    holo_enabled,
                )?;

                let file_tree = write_scaffold_config(file_tree, &TemplateConfig { template })?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(dir! {
                    &name => file_tree
                });

                file_tree.build(&PathBuf::from("."))?;

                let mut nix_instructions = "";

                let app_dir = std::env::current_dir()?.join(&name);
                if setup_nix {
                    if let Err(err) = setup_nix_developer_environment(&app_dir) {
                        fs::remove_dir_all(&app_dir)?;
                        return Err(err)?;
                    }
                    nix_instructions = "nix develop\n";
                }

                if fast_forward {
                    env::set_current_dir(PathBuf::from(&name))?;
                    // prompt to scaffold DNA
                    let dna_name = input_with_case("Initial DNA name (snake_case):", Case::Snake)?;
                    let file_tree = load_directory_into_memory(&current_dir.join(&name))?;
                    let app_file_tree = AppFileTree::get_or_choose(file_tree, &Some(name.clone()))?;
                    let ScaffoldedTemplate { file_tree, .. } =
                        scaffold_dna(app_file_tree, &template_file_tree, &dna_name)?;

                    // prompt to scaffold zome(s)
                    let dna_file_tree = DnaFileTree::get_or_choose(file_tree, &Some(dna_name))?;
                    let (scaffold_integrity, scaffold_coordinator) =
                        select_scaffold_zome_options()?;
                    let name_prompt = match (scaffold_integrity, scaffold_coordinator) {
                        (true, true) => "Enter coordinator zome name (snake_case):\n (The integrity zome will automatically be named '{name of coordinator zome}_integrity')\n",
                        _ => "Enter zome name (snake_case):",
                    };
                    let zome_name = input_with_case(&name_prompt, Case::Snake)?;

                    if scaffold_integrity {
                        let integrity_zome_name = integrity_zome_name(&zome_name);
                        let ScaffoldedTemplate { file_tree, .. } = scaffold_integrity_zome(
                            // FIXME: avoid cloning the dna file tree here
                            dna_file_tree.clone(),
                            &template_file_tree,
                            &integrity_zome_name,
                            &None,
                        )?;
                        file_tree.build(&PathBuf::from("."))?;
                        println!("Integrity zome scaffolded");
                    }

                    if scaffold_coordinator {
                        let ScaffoldedTemplate { file_tree, .. } = scaffold_coordinator_zome(
                            dna_file_tree,
                            &template_file_tree,
                            &zome_name,
                            &None,
                            &None,
                        )?;
                        file_tree.build(&PathBuf::from("."))?;
                        println!("Coordinator zome scaffolded");
                    }
                }

                setup_git_environment(&app_dir)?;

                println!(
                    r#"
Web hApp "{}" scaffolded!
"#,
                    name
                );

                if let Some(i) = next_instructions {
                    println!("{}", i);
                } else {
                    let dna_instructions = if !fast_forward {
                        r#"
- The project won't compile until you add a DNA to it, and then add a zome to that DNA.

To continue scaffolding your application, add new DNAs to your app with:

  hc scaffold dna
"#
                    } else {
                        ""
                    };
                    println!(
                        r#"
Notice that this is an empty skeleton for a Holochain web-app, so:

- The UI is empty, you'll need to import the appropriate components to the top level app component.

Set up your development environment with:

  cd {name} {nix_instructions}
  npm install

{dna_instructions}

Then, at any point in time you can start your application with:

  npm start
"#
                    );
                }
            }
            HcScaffoldCommand::Template(template) => template.run(template_file_tree)?,
            HcScaffoldCommand::Dna { app, name } => {
                let name = match name {
                    Some(n) => {
                        check_case(&n, "dna name", Case::Snake)?;
                        n
                    }
                    None => input_with_case("DNA name (snake_case):", Case::Snake)?,
                };

                let file_tree = load_directory_into_memory(&current_dir)?;

                let app_file_tree = AppFileTree::get_or_choose(file_tree, &app)?;

                let ScaffoldedTemplate {
                    file_tree,
                    next_instructions,
                } = scaffold_dna(app_file_tree, &template_file_tree, &name)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);
                file_tree.build(&PathBuf::from("."))?;

                println!(
                    r#"
DNA "{}" scaffolded!"#,
                    name
                );

                if let Some(i) = next_instructions {
                    println!("{}", i);
                } else {
                    println!(
                        r#"
Add new zomes to your DNA with:

  hc scaffold zome
"#,
                    );
                }
            }
            HcScaffoldCommand::Zome {
                dna,
                name,
                integrity,
                coordinator,
            } => {
                let file_tree = load_directory_into_memory(&current_dir)?;

                if let Some(n) = name.clone() {
                    check_case(&n, "zome name", Case::Snake)?;
                }

                let (scaffold_integrity, scaffold_coordinator) = match (&integrity, &coordinator) {
                    (None, None) => select_scaffold_zome_options()?,
                    _ => (integrity.is_some(), coordinator.is_some()),
                };

                let name_prompt = match (scaffold_integrity, scaffold_coordinator) {
                    (true, true) => String::from("Enter coordinator zome name (snake_case):\n (The integrity zome will automatically be named '{name of coordinator zome}_integrity')\n"),
                    _ => String::from("Enter zome name (snake_case):"),
                };

                let name = match name {
                    Some(n) => n,
                    None => input_with_case(&name_prompt, Case::Snake)?,
                };

                let mut dna_file_tree = DnaFileTree::get_or_choose(file_tree, &dna)?;
                let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

                let mut zome_next_instructions: (Option<String>, Option<String>) = (None, None);

                if scaffold_integrity {
                    let integrity_zome_name = match scaffold_coordinator {
                        true => integrity_zome_name(&name),
                        false => name.clone(),
                    };
                    let ScaffoldedTemplate {
                        file_tree,
                        next_instructions,
                    } = scaffold_integrity_zome(
                        dna_file_tree,
                        &template_file_tree,
                        &integrity_zome_name,
                        &integrity,
                    )?;

                    zome_next_instructions.0 = next_instructions;

                    println!(r#"Integrity zome "{}" scaffolded!"#, integrity_zome_name);

                    dna_file_tree =
                        DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
                }

                if scaffold_coordinator {
                    let dependencies = match scaffold_integrity {
                        true => Some(vec![integrity_zome_name(&name)]),
                        false => {
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
                        &dependencies,
                        &coordinator,
                    )?;
                    zome_next_instructions.1 = next_instructions;

                    println!(r#"Coordinator zome "{}" scaffolded!"#, name);

                    dna_file_tree =
                        DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
                }

                // TODO: implement scaffold_zome_template
                let file_tree =
                    MergeableFileSystemTree::<OsString, String>::from(dna_file_tree.file_tree());

                // FIXME: avoid cloning
                let f = file_tree.clone();
                file_tree.build(&PathBuf::from("."))?;

                // Execute cargo metadata to set up the cargo workspace in case this zome is the first crate
                exec_metadata(&f)?;

                match zome_next_instructions {
                    (Some(integrity), Some(coordinator)) => {
                        println!("{integrity}");
                        println!("{coordinator}");
                    }
                    (None, Some(coordinator)) => println!("{coordinator}"),
                    (Some(integrity), None) => println!("{integrity}"),
                    _ => println!(
                        r#"
Add new entry definitions to your zome with:

  hc scaffold entry-type
"#,
                    ),
                }
            }
            HcScaffoldCommand::EntryType {
                dna,
                zome,
                name,
                crud,
                reference_entry_hash,
                link_from_original_to_each_update,
                fields,
                no_ui,
            } => {
                let file_tree = load_directory_into_memory(&current_dir)?;
                let name = match name {
                    Some(n) => {
                        check_case(&n, "entry type name", Case::Snake)?;
                        n
                    }
                    None => input_with_case("Entry type name (snake_case):", Case::Snake)?,
                };

                let dna_file_tree = DnaFileTree::get_or_choose(file_tree, &dna)?;
                let zome_file_tree = ZomeFileTree::get_or_choose_integrity(dna_file_tree, &zome)?;

                let ScaffoldedTemplate {
                    file_tree,
                    next_instructions,
                } = scaffold_entry_type(
                    zome_file_tree,
                    &template_file_tree,
                    &name,
                    &crud,
                    reference_entry_hash,
                    link_from_original_to_each_update,
                    fields.as_ref(),
                    no_ui,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);
                file_tree.build(&PathBuf::from("."))?;

                println!(
                    r#"
Entry type "{}" scaffolded!"#,
                    name
                );

                if let Some(i) = next_instructions {
                    println!("{}", i);
                } else {
                    println!(
                        r#"
Add new collections for that entry type with:

  hc scaffold collection
"#,
                    );
                }
            }
            HcScaffoldCommand::LinkType {
                dna,
                zome,
                from_referenceable,
                to_referenceable,
                delete,
                bidirectional,
                no_ui,
            } => {
                let file_tree = load_directory_into_memory(&current_dir)?;

                let dna_file_tree = DnaFileTree::get_or_choose(file_tree, &dna)?;
                let zome_file_tree = ZomeFileTree::get_or_choose_integrity(dna_file_tree, &zome)?;

                let ScaffoldedTemplate {
                    file_tree,
                    next_instructions,
                } = scaffold_link_type(
                    zome_file_tree,
                    &template_file_tree,
                    &from_referenceable,
                    &to_referenceable,
                    &delete,
                    &bidirectional,
                    no_ui,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);
                file_tree.build(&PathBuf::from("."))?;

                println!(
                    r#"
Link type scaffolded!
"#,
                );
                if let Some(i) = next_instructions {
                    println!("{}", i);
                }
            }
            HcScaffoldCommand::Collection {
                dna,
                zome,
                collection_name,
                collection_type,
                entry_type,
                no_ui,
            } => {
                let file_tree = load_directory_into_memory(&current_dir)?;

                let dna_file_tree = DnaFileTree::get_or_choose(file_tree, &dna)?;
                let zome_file_tree = ZomeFileTree::get_or_choose_integrity(dna_file_tree, &zome)?;

                let name = match collection_name {
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
                    &template_file_tree,
                    &name,
                    &collection_type,
                    &entry_type,
                    no_ui,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);
                file_tree.build(&PathBuf::from("."))?;

                println!(
                    r#"
Collection "{}" scaffolded!
"#,
                    name
                );

                if let Some(i) = next_instructions {
                    println!("{i}");
                }
            }
            HcScaffoldCommand::Example {
                example,
                holo_enabled,
            } => {
                let example = match example {
                    Some(e) => e,
                    None => choose_example()?,
                };
                let name = example.to_string();

                let app_dir = std::env::current_dir()?.join(&name);
                if app_dir.as_path().exists() {
                    return Err(ScaffoldError::FolderAlreadyExists(app_dir.clone()))?;
                }

                // Match on example types
                let file_tree = match example {
                    Example::HelloWorld => {
                        // scaffold web-app
                        let ScaffoldedTemplate { file_tree, .. } = scaffold_web_app(
                            name.clone(),
                            Some("A simple 'hello world' application.".to_string()),
                            false,
                            &template_file_tree,
                            holo_enabled,
                        )?;

                        file_tree
                    }
                    Example::Forum => {
                        // scaffold web-app
                        let ScaffoldedTemplate { file_tree, .. } = scaffold_web_app(
                            name.clone(),
                            Some("A simple 'forum' application.".to_string()),
                            false,
                            &template_file_tree,
                            holo_enabled,
                        )?;

                        // scaffold dna hello_world
                        let dna_name = "forum";

                        let app_file_tree = AppFileTree::get_or_choose(file_tree, &Some(name))?;
                        let ScaffoldedTemplate { file_tree, .. } =
                            scaffold_dna(app_file_tree, &template_file_tree, dna_name)?;

                        // scaffold integrity zome posts
                        let dna_file_tree =
                            DnaFileTree::get_or_choose(file_tree, &Some(dna_name.to_string()))?;
                        let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

                        let integrity_zome_name = "posts_integrity";
                        let integrity_zome_path = PathBuf::new()
                            .join("dnas")
                            .join(dna_name)
                            .join("zomes")
                            .join("integrity");
                        let ScaffoldedTemplate { file_tree, .. } =
                            scaffold_integrity_zome_with_path(
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
                        let ScaffoldedTemplate { file_tree, .. } =
                            scaffold_coordinator_zome_in_path(
                                dna_file_tree,
                                &template_file_tree,
                                coordinator_zome_name,
                                &Some(vec![integrity_zome_name.to_string()]),
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
                            &Some(integrity_zome_name.to_string()),
                        )?;

                        let post_entry_type_name = "post";

                        let ScaffoldedTemplate { file_tree, .. } = scaffold_entry_type(
                            zome_file_tree,
                            &template_file_tree,
                            post_entry_type_name,
                            &Some(Crud {
                                update: true,
                                delete: true,
                            }),
                            Some(false),
                            Some(true),
                            Some(&vec![
                                FieldDefinition {
                                    field_name: String::from("title"),
                                    field_type: FieldType::String,
                                    widget: Some(String::from("TextField")),
                                    cardinality: Cardinality::Single,
                                    linked_from: None,
                                },
                                FieldDefinition {
                                    field_name: String::from("content"),
                                    field_type: FieldType::String,
                                    widget: Some(String::from("TextArea")),
                                    cardinality: Cardinality::Single,
                                    linked_from: None,
                                },
                            ]),
                            false,
                        )?;

                        let dna_file_tree =
                            DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

                        let zome_file_tree = ZomeFileTree::get_or_choose_integrity(
                            dna_file_tree,
                            &Some(integrity_zome_name.to_string()),
                        )?;

                        let comment_entry_type_name = "comment";

                        let ScaffoldedTemplate { file_tree, .. } = scaffold_entry_type(
                            zome_file_tree,
                            &template_file_tree,
                            comment_entry_type_name,
                            &Some(Crud {
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
                                    linked_from: Some(Referenceable::EntryType(
                                        EntryTypeReference {
                                            entry_type: post_entry_type_name.to_string(),
                                            reference_entry_hash: false,
                                        },
                                    )),
                                },
                            ]),
                            false,
                        )?;

                        let dna_file_tree =
                            DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

                        let zome_file_tree = ZomeFileTree::get_or_choose_integrity(
                            dna_file_tree,
                            &Some(integrity_zome_name.to_string()),
                        )?;

                        let ScaffoldedTemplate { file_tree, .. } = scaffold_collection(
                            zome_file_tree,
                            &template_file_tree,
                            &String::from("all_posts".to_string()),
                            &Some(CollectionType::Global),
                            &Some(EntryTypeReference {
                                entry_type: post_entry_type_name.to_string(),
                                reference_entry_hash: false,
                            }),
                            false,
                        )?;

                        file_tree
                    }
                };

                let ScaffoldedTemplate {
                    file_tree,
                    next_instructions,
                } = scaffold_example(file_tree, &template_file_tree, &example)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);
                file_tree.build(&app_dir)?;

                // set up nix
                if let Err(err) = setup_nix_developer_environment(&app_dir) {
                    fs::remove_dir_all(&app_dir)?;
                    return Err(err)?;
                }

                setup_git_environment(&app_dir)?;

                println!(
                    r#"
Example "{}" scaffolded!
"#,
                    example.to_string()
                );

                if let Some(i) = next_instructions {
                    println!("{}", i);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffoldTemplate {
    /// Clone the template in use into a new custom template
    Clone {
        #[structopt(long)]
        /// The folder to initialize the template into, will end up at "<TO TEMPLATE>"
        to_template: Option<String>,
    },
}

impl HcScaffoldTemplate {
    pub fn run(self, template_file_tree: FileTree) -> anyhow::Result<()> {
        let target_template = match self.target_template() {
            Some(t) => t,
            None => {
                // Enter template name
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter new template name:")
                    .interact()?
            }
        };

        let template_file_tree = dir! {
            target_template.clone() => template_file_tree
        };

        let file_tree = MergeableFileSystemTree::<OsString, String>::from(template_file_tree);

        file_tree.build(&PathBuf::from("."))?;

        match self {
            HcScaffoldTemplate::Clone { .. } => {
                println!(
                    r#"Template initialized to folder {:?}
"#,
                    target_template
                );
            }
        }
        Ok(())
    }

    pub fn target_template(&self) -> Option<String> {
        match self {
            HcScaffoldTemplate::Clone {
                to_template: target_template,
                ..
            } => target_template.clone(),
        }
    }
}

fn setup_git_environment(path: &Path) -> ScaffoldResult<()> {
    let output = Command::new("git")
        .stdout(Stdio::inherit())
        .current_dir(path)
        .args(["init", "--initial-branch=main"])
        .output()?;

    if !output.status.success() {
        let output = Command::new("git")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .current_dir(path)
            .args(["init"])
            .output()?;
        if !output.status.success() {
            println!("Warning: error running \"git init\"");
            return Ok(());
        }

        let _output = Command::new("git")
            .current_dir(path)
            .args(["branch", "main"])
            .output()?;
    }

    let output = Command::new("git")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .args(["add", "."])
        .output()?;

    if !output.status.success() {
        println!("Warning: error running \"git add .\"");
    }
    Ok(())
}

/// Write hcScaffold config to the hApp's root `package.json` file
fn write_scaffold_config(
    mut web_app_file_tree: FileTree,
    config: &TemplateConfig,
) -> ScaffoldResult<FileTree> {
    if Path::new(&config.template).exists() {
        return Ok(web_app_file_tree);
    }
    let package_json_path = PathBuf::from("package.json");
    map_file(&mut web_app_file_tree, &package_json_path, |c| {
        let original_content = c.clone();
        let json = serde_json::from_str::<Value>(&c)?;
        let json = match json {
            Value::Object(mut o) => {
                o.insert(
                    "hcScaffold".to_owned(),
                    serde_json::to_value(config).unwrap(),
                );
                o
            }
            _ => return Ok(original_content),
        };
        let json = serde_json::to_value(json)?;
        let json = serde_json::to_string_pretty(&json)?;
        Ok(json)
    })?;
    Ok(web_app_file_tree)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConfig {
    template: String,
}

/// Gets template config written to the root `package.json` file when the hApp was
/// originally scaffolded
fn get_template_config(current_dir: &Path) -> ScaffoldResult<Option<TemplateConfig>> {
    let package_json_path = current_dir.join("package.json");
    let Ok(file) = fs::read_to_string(package_json_path) else {
        return Ok(None);
    };
    let file = serde_json::from_str::<Value>(&file)?;
    if let Some(config) = file.get("hcScaffold") {
        let config = serde_json::from_value(config.to_owned())?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}
