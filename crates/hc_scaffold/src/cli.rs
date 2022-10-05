use crate::{error::ScaffoldResult, generators};
use std::{path::PathBuf, process::Command, time::Duration};
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
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            HcScaffold::WebApp { name, description } => {
                generators::web_app::scaffold_web_app(name.clone(), description)?;

                if cfg!(target_os = "windows") {
                    return Err(anyhow::anyhow!("Windows doesn't support nix"));
                } else {
                    Command::new("nix-shell")
                        .current_dir(std::env::current_dir()?.join(name))
                        .args(["-I", "nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz", "-p", "niv", "--run", "niv init && niv drop nixpkgs && niv drop niv && niv add -b main holochain/holonix"])
                        .output()?;
                };
            }
            HcScaffold::Pack(Pack::WebApp { path }) => {
                // Go through apps
                // Go through DNAs
                // Pack DNAs
                // Pack apps
                // Pack web-app
            }
            HcScaffold::Pack(Pack::App { path }) => {
                // Go through DNAs
                // Pack DNAs
                // Pack app
            }
        }

        Ok(())
    }
}
