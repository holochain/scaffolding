use crate::definitions::FieldType;
use crate::error::ScaffoldError;
use crate::file_tree::load_directory_into_memory;
use crate::generators::app::cargo::exec_metadata;
use crate::generators::index::{scaffold_index, IndexType};
use crate::generators::link_type::scaffold_link_type;
use crate::generators::web_app::uis::UiFramework;
use crate::generators::{
    self,
    app::utils::get_or_choose_app_manifest,
    dna::{scaffold_dna, utils::get_or_choose_dna_manifest},
    entry_def::scaffold_entry_def,
    zome::{
        integrity_zome_name, scaffold_coordinator_zome, scaffold_integrity_zome,
        scaffold_zome_pair, utils::get_or_choose_integrity_zome, utils::select_integrity_zomes,
    },
};
use crate::utils::{
    check_no_whitespace, check_snake_case, input_no_whitespace, input_snake_case, input_yes_or_no,
};

use build_fs_tree::{Build, MergeableFileSystemTree};
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::BTreeMap;
use std::process::Stdio;
use std::{ffi::OsString, path::PathBuf, process::Command};
use structopt::StructOpt;

/// The list of subcommands for `hc scaffold`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffold {
    /// Scaffold a new, empty web app
    WebApp {
        /// Name of the app to scaffold
        name: Option<String>,

        /// [OPTIONAL] Description of the app to scaffold
        description: Option<String>,

        #[structopt(long)]
        /// Whether to setup the holonix development environment for this web-app
        setup_nix: Option<bool>,

        #[structopt(long)]
        /// The UI framework to use as the template for this web-app
        ui: Option<UiFramework>,
    },
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
        /// Name of the app in which you want to scaffold the zome
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        /// Name of the zome being scaffolded
        name: Option<String>,

        #[structopt(long)]
        /// The path in which you want to scaffold the zome
        path: Option<PathBuf>,

        #[structopt(long)]
        /// create an integrity zome only
        integrity: bool,

        #[structopt(long)]
        /// create a coordinator zome only
        coordinator: bool,
    },
    /// Scaffold an entry type and CRUD functions into an existing zome
    EntryType {
        #[structopt(long)]
        /// Name of the app in which you want to scaffold the zome
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the entry definition
        zome: Option<String>,

        /// Name of the entry type being scaffolded
        name: Option<String>,

        #[structopt(long, parse(try_from_str = parse_crud))]
        /// Whether to create a read zome call function for this entry type
        crud: Option<Crud>,

        #[structopt(long, value_delimiter = ",")]
        /// The entry types that the new entry type depends on
        depends_on: Option<Vec<String>>,

        #[structopt(long, value_delimiter = ",", parse(try_from_str = parse_fields))]
        /// The fields that the entry type struct should contain
        fields: Option<Vec<(String, FieldType)>>,
    },
    /// Scaffold a link type and its appropriate zome functions into an existing zome
    LinkType {
        #[structopt(long)]
        /// Name of the app in which you want to scaffold the zome
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the link type
        zome: Option<String>,

        /// Entry type used as the base for the links
        from_entry_type: Option<String>,

        /// Entry type used as the target for the links
        to_entry_type: Option<String>,

        #[structopt(long)]
        /// Use the entry hash as the base for the links, instead of the action hash
        link_from_entry_hash: bool,

        #[structopt(long)]
        /// Use the entry hash as the target for the links, instead of the action hash
        link_to_entry_hash: bool,
    },
    /// Scaffold an indexing link-type and appropriate zome functions to index entries into an existing zome
    Index {
        #[structopt(long)]
        /// Name of the app in which you want to scaffold the zome
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the link type
        zome: Option<String>,

        /// Index type: "global" or "by-author"
        index_type: Option<IndexType>,

        /// Index name, just to differentiate it from other indexes
        index_name: Option<String>,

        #[structopt(long, value_delimiter = ",")]
        /// Entry types that are going to be indexed by this index
        entry_types: Option<Vec<String>>,

        #[structopt(long)]
        /// Use the entry hash as the target for the links, instead of the action hash
        link_to_entry_hash: bool,
    },
}

pub fn parse_fields(_fields_str: &str) -> Result<(String, FieldType), String> {
    Err(String::from("TODO!"))
}

#[derive(Debug, Clone)]
pub struct Crud {
    // We don't include create because create must always exist
    pub read: bool,
    pub update: bool,
    pub delete: bool,
}

fn parse_crud(crud_str: &str) -> Result<Crud, String> {
    if !crud_str.contains('c') {
        return Err(String::from("create ('c') must be present"));
    }

    let mut crud = Crud {
        read: false,
        update: false,
        delete: false,
    };

    for c in crud_str.chars() {
        match c {
            'c' => {}
            'r' => {
                crud.read = true;
            }
            'u' => {
                crud.update = true;
            }
            'd' => {
                crud.delete = true;
            }
            _ => {
                return Err(String::from(
                    "Only 'c', 'r', 'u' and 'd' are allowed in the crud argument",
                ));
            }
        }
    }

    Ok(crud)
}

impl HcScaffold {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            HcScaffold::WebApp {
                name,
                description,
                setup_nix,
                ui,
            } => {
                let prompt = String::from("App name (no whitespaces):");
                let name: String = match name {
                    Some(n) => check_no_whitespace(n, "app name")?,
                    None => input_no_whitespace(&prompt)?,
                };

                let setup_nix = match setup_nix {
                    Some(s) => s,
                    None => {
                        let holonix_prompt = String::from("Do you want to set up the holonix development environment for this project?");
                        input_yes_or_no(&holonix_prompt, Some(true))?
                    }
                };

                let app_file_tree = generators::web_app::scaffold_web_app(
                    name.clone(),
                    description,
                    !setup_nix,
                    &ui,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                let mut maybe_nix = "";

                if setup_nix {
                    if cfg!(target_os = "windows") {
                        return Err(anyhow::anyhow!("Windows doesn't support nix"));
                    } else {
                        Command::new("nix-shell")
                        .stdout(Stdio::inherit())
                        .current_dir(std::env::current_dir()?.join(&name))
                        .args(["-I", "nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz", "-p", "niv", "--run", "niv init && niv drop nixpkgs && niv drop niv && niv add -b main holochain/holonix"])
                        .output()?;
                        maybe_nix = "\n  nix-shell";
                    };
                }

                println!(
                    r#"
Web hApp "{}" scaffolded!

To set up your development environment, run:

  cd {}{}
  npm install

Then, add new DNAs to your app with:

  hc-scaffold dna
"#,
                    name, name, maybe_nix
                );
            }
            HcScaffold::Dna { app, name } => {
                let prompt = String::from("DNA name (snake_case):");
                let name: String = match name {
                    Some(n) => check_snake_case(n, "dna name")?,
                    None => input_snake_case(&prompt)?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;
                let file_tree = scaffold_dna(app_file_tree, &app, &name)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"
DNA "{}" scaffolded!

Add new zomes to your DNA with:

  hc-scaffold zome
"#,
                    name
                );
            }
            HcScaffold::Zome {
                app,
                dna,
                name,
                path,
                integrity,
                coordinator,
            } => {
                if let Some(n) = name.clone() {
                    check_snake_case(n, "zome name")?;
                }

                let selected_option =
                    match (integrity, coordinator) {
                        (false, false) => Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("What do you want to add?")
                            .default(0)
                            .items(&[
                                "integrity/coordinator zome-pair",
                                "integrity zome",
                                "coordinator zome",
                            ])
                            .interact()?,
                        (true, false) => 1,
                        (false, true) => 2,
                        (true, true) => return Err(anyhow::Error::from(
                            ScaffoldError::InvalidArguments(String::from(
                                "The --integrity and --coordinator flags are mutually exclusive.",
                            )),
                        )),
                    };

                let name_prompt = match selected_option {
                    0 => String::from("Coordinator zome name (snake_case):\n (The integrity zome will automatically be named '{name of coordinator zome}_integrity')\n"),
                    _ => String::from("Enter zome name (snake_case):"),
                };

                let name: String = match name {
                    Some(n) => n,
                    None => input_snake_case(&name_prompt)?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;
                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (dna_manifest_path, dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let mut dependencies = None;
                if selected_option == 2 {
                    dependencies = Some(select_integrity_zomes(&dna_manifest, Some(&String::from(
                        "Select integrity zome(s) this coordinator zome depends on (SPACE to select/unselect, ENTER to continue):"
                        )))?
                    );
                }

                let app_file_tree = match selected_option {
                    1 => scaffold_integrity_zome(app_file_tree, &dna_manifest_path, &name, &path)?,
                    2 => scaffold_coordinator_zome(
                        app_file_tree,
                        &dna_manifest_path,
                        &name,
                        &dependencies,
                        &path,
                    )?,
                    _ => scaffold_zome_pair(app_file_tree, &dna_manifest_path, &name, &path)?,
                };

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                let f = file_tree.clone();

                file_tree.build(&".".into())?;

                // Execute cargo metadata to set up the cargo workspace in case this zome is the first crate
                exec_metadata(&f)?;

                let headline = match selected_option {
                    1 => format!(r#"Integrity zome "{}" scaffolded!"#, name),
                    2 => format!(r#"Coordinator zome "{}" scaffolded!"#, name),
                    _ => format!(
                        r#"Integrity zome "{}" and coordinator zome "{}" scaffolded!"#,
                        integrity_zome_name(&name),
                        name
                    ),
                };

                println!(
                    r#"
{}

Add new entry definitions to your zome with:

  hc-scaffold entry-type
"#,
                    headline
                );
            }
            HcScaffold::EntryType {
                app,
                dna,
                zome,
                name,
                crud,
                depends_on,
                fields,
            } => {
                let prompt = String::from("Entry definition name (snake_case):");
                let name: String = match name {
                    Some(n) => check_snake_case(n, "entry definition name")?,
                    None => input_snake_case(&prompt)?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (_dna_manifest_path, dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let integrity_zome_name = get_or_choose_integrity_zome(&dna_manifest, &zome)?;

                let fields: Option<BTreeMap<String, FieldType>> =
                    fields.map(|f| f.into_iter().collect());

                let app_file_tree = scaffold_entry_def(
                    app_file_tree,
                    &app_manifest,
                    &dna_manifest,
                    &integrity_zome_name,
                    &name,
                    &crud,
                    &depends_on,
                    &fields,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"
Entry type "{}" scaffolded!

Add new indexes for that entry type with:

  hc-scaffold index
"#,
                    name
                );
            }
            HcScaffold::LinkType {
                app,
                dna,
                zome,
                from_entry_type,
                to_entry_type,
                link_from_entry_hash,
                link_to_entry_hash,
            } => {
                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (_dna_manifest_path, dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let integrity_zome_name = get_or_choose_integrity_zome(&dna_manifest, &zome)?;

                let (app_file_tree, link_type_name) = scaffold_link_type(
                    app_file_tree,
                    &dna_manifest,
                    &integrity_zome_name,
                    &from_entry_type,
                    &to_entry_type,
                    link_from_entry_hash,
                    link_to_entry_hash,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"
Link type "{}" scaffolded!
"#,
                    link_type_name
                );
            }
            HcScaffold::Index {
                app,
                dna,
                zome,
                index_name,
                index_type,
                entry_types,
                link_to_entry_hash,
            } => {
                let prompt = String::from("Index name (snake_case, eg. \"all_posts\"):");
                let name: String = match index_name {
                    Some(n) => check_snake_case(n, "index name")?,
                    None => input_snake_case(&prompt)?,
                };

                let current_dir = std::env::current_dir()?;
                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (_dna_manifest_path, dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let integrity_zome_name = get_or_choose_integrity_zome(&dna_manifest, &zome)?;

                let app_file_tree = scaffold_index(
                    app_file_tree,
                    &dna_manifest,
                    &integrity_zome_name,
                    &name,
                    &index_type,
                    &entry_types,
                    link_to_entry_hash,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"
Index "{}" scaffolded!
"#,
                    name
                );
            }
        }

        Ok(())
    }
}
