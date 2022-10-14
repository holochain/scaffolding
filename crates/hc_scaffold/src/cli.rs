use crate::{
    generators::{
        self,
        app::utils::{bundled_dnas_locations, get_or_choose_app_manifest},
        dna::{scaffold_dna, utils::get_or_choose_dna_manifest},
        zome::{scaffold_coordinator_zome, scaffold_integrity_zome, scaffold_zome_pair},
    },
    utils::choose_directory_path,
};
use build_fs_tree::{Build, MergeableFileSystemTree};
use holochain_scaffolding_utils::load_directory_into_memory;
use holochain_types::{prelude::AppManifest, web_app::WebAppManifest};
use holochain_util::ffs;
use mr_bundle::{Location, Manifest};
use std::{ffi::OsString, path::PathBuf, process::Command};
use structopt::StructOpt;

/// The list of subcommands for `hc sandbox`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffold {
    /// Scaffold a new web app
    WebApp {
        /// Name of the app to scaffold
        name: String,

        /// [OPTIONAL] Description of the app to scaffold
        description: Option<String>,
    },
    /// Scaffold a DNA into an existing app
    Dna {
        #[structopt(long)]
        /// Name of the app you want to scaffold the DNA into
        app: Option<String>,

        /// Name of the DNA being scaffolded
        name: String,
    },
    /// Scaffold an integrity-coordinator zome pair into an existing DNA
    Zome {
        #[structopt(long)]
        /// Name of the app you want to scaffold the zome into
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna you want to scaffold the zome into
        dna: Option<String>,

        /// Name of the zome being scaffolded
        name: String,

        #[structopt(long)]
        /// The path in which you want to scaffold the zome
        path: Option<PathBuf>,
    },
    /// Scaffold a zome into an existing DNA
    IntegrityZome {
        #[structopt(long)]
        /// Name of the app you want to scaffold the zome into
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna you want to scaffold the zome into
        dna: Option<String>,

        /// Name of the zome being scaffolded
        name: String,

        #[structopt(long)]
        /// The path in which you want to scaffold the zome
        path: Option<PathBuf>,
    },
    /// Scaffold a zome into an existing DNA
    CoordinatorZome {
        #[structopt(long)]
        /// Name of the app you want to scaffold the zome into
        app: Option<String>,

        #[structopt(long)]
        /// Name of the dna you want to scaffold the zome into
        dna: Option<String>,

        /// Name of the zome being scaffolded
        name: String,

        #[structopt(long)]
        /// The path in which you want to scaffold the zome
        path: Option<PathBuf>,

        #[structopt(long, value_delimiter = ",")]
        dependencies: Option<Vec<String>>,
    },
    Pack(Pack),
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
                let app_file_tree =
                    generators::web_app::scaffold_web_app(name.clone(), description)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(app_file_tree);

                file_tree.build(&".".into())?;

                if cfg!(target_os = "windows") {
                    return Err(anyhow::anyhow!("Windows doesn't support nix"));
                } else {
                    Command::new("nix-shell")
                        .current_dir(std::env::current_dir()?.join(name))
                        .args(["-I", "nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz", "-p", "niv", "--run", "niv init && niv drop nixpkgs && niv drop niv && niv add -b main holochain/holonix"])
                        .output()?;
                };
            }
            HcScaffold::Dna { app, name } => {
                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;
                let file_tree = scaffold_dna(app_file_tree, app, name)?;

                let file_tree = MergeableFileSystemTree::<OsString, String>::from(file_tree);

                file_tree.build(&".".into())?;
            }
            HcScaffold::Zome {
                app,
                dna,
                name,
                path,
            } => {
                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;
                let app_manifest = get_or_choose_app_manifest(&app_file_tree, app)?;
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
            }
            HcScaffold::IntegrityZome {
                app,
                dna,
                name,
                path,
            } => {
                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, app)?;
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
            }
            HcScaffold::CoordinatorZome {
                app,
                dna,
                name,
                dependencies,
                path,
            } => {
                let current_dir = std::env::current_dir()?;

                let app_file_tree = load_directory_into_memory(&current_dir)?;

                let app_manifest = get_or_choose_app_manifest(&app_file_tree, app)?;
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
