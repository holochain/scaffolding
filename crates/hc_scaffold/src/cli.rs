use crate::definitions::FieldType;
use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::load_directory_into_memory;
use crate::generators::app::cargo::exec_metadata;
use crate::generators::index::{scaffold_index, IndexType};
use crate::generators::link_type::scaffold_link_type;
use crate::generators::{
    self,
    app::utils::{bundled_dnas_locations, get_or_choose_app_manifest},
    dna::{scaffold_dna, utils::get_or_choose_dna_manifest},
    entry_def::scaffold_entry_def,
    zome::{
        integrity_zome_name, scaffold_coordinator_zome, scaffold_integrity_zome,
        scaffold_zome_pair, utils::get_or_choose_integrity_zome,
    },
};
use crate::utils::{check_no_whitespace, check_snake_case, input_no_whitespace, input_snake_case, input_yes_or_no};

use build_fs_tree::{Build, MergeableFileSystemTree};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use holochain_types::{prelude::AppManifest, web_app::WebAppManifest};
use holochain_util::ffs;
use mr_bundle::{Location, Manifest};
use std::collections::BTreeMap;
use std::process::Stdio;
use std::str::FromStr;
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
        /// Skip setup of nix development environment
        skip_nix: bool,
    },
    /// Scaffold a DNA into an existing app
    Dna {
        #[structopt(long)]
        /// Name of the app in which you want to scaffold the DNA
        app: Option<String>,

        /// Name of the DNA being scaffolded
        name: Option<String>,
    },
    /// Scaffold an integrity-coordinator zome pair into an existing DNA
    Zomes {
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
    },
    /// Scaffold an integrity zome into an existing DNA
    IntegrityZome {
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
    },
    /// Scaffold a coordinator zome into an existing DNA
    CoordinatorZome {
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

        #[structopt(long, value_delimiter = ",")]
        /// The integrity zome dependencies for the coordinator zome
        dependencies: Option<Vec<String>>,
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
    Pack(Pack),
}

pub fn parse_fields(fields_str: &str) -> Result<(String, FieldType), String> {
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

/// Packaging of apps
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum Pack {
    /// Package a web app
    WebApp {
        /// The path to the working directory containing a `web-happ.yaml` manifest
        path: PathBuf,
    },
    /// Package an app
    App {
        /// The path to the working directory containing a `happ.yaml` manifest
        path: PathBuf,
    },
}

impl HcScaffold {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            HcScaffold::WebApp {
                name,
                description,
                mut skip_nix,
            } => {
                let prompt = String::from("App name (no whitespaces):");
                let name: String = match name {
                    Some(n) => check_no_whitespace(n, "app name")?,
                    None => input_no_whitespace(&prompt)?,
                };

                if !skip_nix {
                    let holonix_prompt = String::from("Do you want to set up the holonix development environment for this project?");
                    skip_nix = input_yes_or_no(&holonix_prompt, Some(true))?;
                }

                let app_file_tree =
                    generators::web_app::scaffold_web_app(name.clone(), description, skip_nix)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                let mut maybe_nix = "";

                if !skip_nix {
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

  hc-scaffold zomes
"#,
                    name
                );
            }
            HcScaffold::Zomes {
                app,
                dna,
                name,
                path,
            } => {
                let prompt = String::from("Zome name (snake_case):");
                let name: String = match name {
                    Some(n) => check_snake_case(n, "zome names")?,
                    None => input_snake_case(&prompt)?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;
                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (dna_manifest_path, _dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let app_file_tree = scaffold_zome_pair(
                    app_file_tree,
                    &app_manifest.1,
                    &dna_manifest_path,
                    &name,
                    &String::from("0.1"),
                    &String::from("0.0.155"),
                    &path,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                let f = file_tree.clone();

                file_tree.build(&".".into())?;

                // Execute cargo metadata to set up the cargo workspace in case this zome is the first crate
                exec_metadata(&f)?;

                println!(
                    r#"
Integrity zome "{}" and coordinator zome "{}" scaffolded!

Add new entry definitions to your zome with:

  hc-scaffold entry-type
"#,
                    integrity_zome_name(&name),
                    name
                );
            }
            HcScaffold::IntegrityZome {
                app,
                dna,
                name,
                path,
            } => {
                let prompt = String::from("Integrity zome name (snake_case):");
                let name: String = match name {
                    Some(n) => check_snake_case(n, "zome names")?,
                    None => input_snake_case(&prompt)?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (dna_manifest_path, _dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let app_file_tree = scaffold_integrity_zome(
                    app_file_tree,
                    &app_manifest.1,
                    &dna_manifest_path,
                    &name,
                    &String::from("0.1.0"),
                    &path,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                let f = file_tree.clone();

                file_tree.build(&".".into())?;

                // Execute cargo metadata to set up the cargo workspace in case this zome is the first crate
                exec_metadata(&f)?;

                println!(
                    r#"
Integrity zome "{}" scaffolded!

Add new entry definitions to your zome with:

  hc-scaffold entry_def
"#,
                    name
                );
            }
            HcScaffold::CoordinatorZome {
                app,
                dna,
                name,
                dependencies,
                path,
            } => {
                let prompt = String::from("Coordinator zome name (snake_case):");
                let name: String = match name {
                    Some(n) => check_snake_case(n, "zome names")?,
                    None => input_snake_case(&prompt)?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, &app)?;
                let (dna_manifest_path, _dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;
                let app_file_tree = scaffold_coordinator_zome(
                    app_file_tree,
                    &app_manifest.1,
                    &dna_manifest_path,
                    &name,
                    &String::from("0.0.155"),
                    &dependencies,
                    &path,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                let f = file_tree.clone();

                file_tree.build(&".".into())?;

                // Execute cargo metadata to set up the cargo workspace in case this zome is the first crate
                exec_metadata(&f)?;

                println!(
                    r#"
Coordinator zome "{}" scaffolded!

Add new entry definitions to your zome with:

  hc-scaffold entry_def
"#,
                    name
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
                let (dna_manifest_path, dna_manifest) =
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
                let (dna_manifest_path, dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let integrity_zome_name = get_or_choose_integrity_zome(&dna_manifest, &zome)?;

                let (app_file_tree, link_type_name) = scaffold_link_type(
                    app_file_tree,
                    &app_manifest,
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
                let (dna_manifest_path, dna_manifest) =
                    get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna)?;

                let integrity_zome_name = get_or_choose_integrity_zome(&dna_manifest, &zome)?;

                let app_file_tree = scaffold_index(
                    app_file_tree,
                    &app_manifest,
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
            HcScaffold::Pack(Pack::WebApp { path }) => web_app_pack_all_bundled(path).await?,
            HcScaffold::Pack(Pack::App { path }) => app_pack_all_bundled(path).await?,
        }

        Ok(())
    }
}

async fn web_app_pack_all_bundled(web_app_bundle_path: PathBuf) -> anyhow::Result<()> {
    let web_app_bundle_path = ffs::canonicalize(web_app_bundle_path).await?;

    let f = std::fs::File::open(web_app_bundle_path.join(WebAppManifest::path()))?;

    let manifest: WebAppManifest = serde_yaml::from_reader(f)?;

    let location = manifest.happ_bundle_location();

    if let Location::Bundled(mut bundled_location) = location {
        bundled_location.pop();
        bundled_location = PathBuf::new()
            .join(web_app_bundle_path.clone())
            .join(bundled_location);

        app_pack_all_bundled(bundled_location).await?;
    }

    holochain_cli_bundle::HcWebAppBundle::Pack {
        path: web_app_bundle_path,
        output: None,
    }
    .run()
    .await?;

    Ok(())
}

async fn app_pack_all_bundled(app_workdir_path: PathBuf) -> anyhow::Result<()> {
    let app_workdir_path = ffs::canonicalize(app_workdir_path).await?;

    let app_manifest_path = app_workdir_path.join(AppManifest::path());
    let f = std::fs::File::open(&app_manifest_path)?;

    let manifest: AppManifest = serde_yaml::from_reader(f)?;

    let dna_locations = bundled_dnas_locations(&app_manifest_path, &manifest);

    for bundled_location in dna_locations {
        holochain_cli_bundle::HcDnaBundle::Pack {
            path: ffs::canonicalize(bundled_location).await?,
            output: None,
        }
        .run()
        .await?;
    }

    holochain_cli_bundle::HcAppBundle::Pack {
        path: app_workdir_path,
        output: None,
    }
    .run()
    .await?;

    Ok(())
}
