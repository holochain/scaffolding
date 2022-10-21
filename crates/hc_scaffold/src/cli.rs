use crate::definitions::FieldType;
use crate::file_tree::load_directory_into_memory;
use crate::{
    generators::{
        self,
        app::utils::{bundled_dnas_locations, get_or_choose_app_manifest},
        dna::{scaffold_dna, utils::get_or_choose_dna_manifest},
        entry_def::scaffold_entry_def,
        zome::{
            integrity_zome_name, scaffold_coordinator_zome, scaffold_integrity_zome,
            scaffold_zome_pair, utils::get_or_choose_integrity_zome,
        },
    },
    utils::choose_directory_path,
};
use build_fs_tree::{Build, MergeableFileSystemTree};
use dialoguer::{theme::ColorfulTheme, Input};
use holochain_types::{prelude::AppManifest, web_app::WebAppManifest};
use holochain_util::ffs;
use mr_bundle::{Location, Manifest};
use std::collections::BTreeMap;
use std::{ffi::OsString, path::PathBuf, process::Command};
use structopt::StructOpt;

/// The list of subcommands for `hc sandbox`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffold {
    /// Scaffold a new web app
    WebApp {
        /// Name of the app to scaffold
        name: Option<String>,

        /// [OPTIONAL] Description of the app to scaffold
        description: Option<String>,
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

    /// Scaffold an integrity zome into an existing DNA
    EntryDef {
        #[structopt(long)]
        /// Name of the app in which you want to scaffold the zome
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna in which you want to scaffold the zome
        dna: Option<String>,

        #[structopt(long)]
        /// Name of the integrity zome in which you want to scaffold the entry definition
        zome: Option<String>,

        /// Name of the zome being scaffolded
        name: Option<String>,

        #[structopt(long)]
        /// The path in which you want to scaffold the zome
        path: Option<PathBuf>,

        #[structopt(long, parse(try_from_str = parse_crud))]
        /// Whether to create a read zome call function for this entry definition
        crud: Option<Crud>,

        #[structopt(long, value_delimiter = ",", parse(try_from_str = parse_fields))]
        fields: Option<Vec<(String, FieldType)>>,
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

/// The list of subcommands for `hc sandbox`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum Pack {
    /// Scaffold a new web app
    WebApp {
        /// The path to the working directory containing a `web-happ.yaml` manifest
        path: PathBuf,
    },
    App {
        /// The path to the working directory containing a `happ.yaml` manifest
        path: PathBuf,
    },
}

impl HcScaffold {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            HcScaffold::WebApp { name, description } => {
                let name: String = match name {
                    Some(n) => n,
                    None => Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("App name:")
                        .interact_text()?,
                };

                let app_file_tree =
                    generators::web_app::scaffold_web_app(name.clone(), description)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                if cfg!(target_os = "windows") {
                    return Err(anyhow::anyhow!("Windows doesn't support nix"));
                } else {
                    Command::new("nix-shell")
                        .current_dir(std::env::current_dir()?.join(&name))
                        .args(["-I", "nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz", "-p", "niv", "--run", "niv init && niv drop nixpkgs && niv drop niv && niv add -b main holochain/holonix"])
                        .output()?;
                };

                println!(
                    r#"Web hApp "{}" scaffolded!

To set up your development environment, run:

  cd {}
  nix-shell
  npm install

Then, add new DNAs to your app with:

  hc-scaffold dna
"#,
                    name, name
                );
            }
            HcScaffold::Dna { app, name } => {
                let name: String = match name {
                    Some(n) => n,
                    None => Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("DNA name:")
                        .interact_text()?,
                };

                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;
                let file_tree = scaffold_dna(app_file_tree, &app, &name)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"DNA "{}" scaffolded!

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
            } => {
                let name: String = match name {
                    Some(n) => n,
                    None => Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Zome name:")
                        .interact_text()?,
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
                    &String::from("0.1.0"),
                    &String::from("0.0.155"),
                    &path,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"Integrity zome "{}" and coordinator zome "{}" scaffolded!

Warning: right now the application won't compile because the scaffolded integrity zome has no entry definitions.
Add new entry definitions to your zome with:

  hc-scaffold entry-def
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
                let name: String = match name {
                    Some(n) => n,
                    None => Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Integrity zome name:")
                        .interact_text()?,
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

                file_tree.build(&".".into())?;

                println!(
                    r#"Integrity zome "{}" scaffolded!

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
                let name: String = match name {
                    Some(n) => n,
                    None => Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Coordinator zome name:")
                        .interact_text()?,
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

                file_tree.build(&".".into())?;

                println!(
                    r#"Coordinator zome "{}" scaffolded!

Add new entry definitions to your zome with:

  hc-scaffold entry_def
"#,
                    name
                );
            }
            HcScaffold::EntryDef {
                app,
                dna,
                zome,
                name,
                path,
                crud,
                fields,
            } => {
                let name: String = match name {
                    Some(n) => n,
                    None => Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Entry definition name:")
                        .interact_text()?,
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
                    &app_manifest.1,
                    &dna_manifest,
                    &integrity_zome_name,
                    &name,
                    &crud,
                    &fields,
                )?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                println!(
                    r#"Entry definition "{}" scaffolded!

Add new entry definitions to your zome with:

  hc-scaffold entry_def
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
