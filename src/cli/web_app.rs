use std::{path::PathBuf, str::FromStr};

use colored::Colorize;
use structopt::StructOpt;
use tokio::fs;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{build_file_tree, file_content, FileTree},
    scaffold::{
        app::{git::setup_git_environment, nix::setup_nix_developer_environment},
        web_app::{package_manager::PackageManager, scaffold_web_app},
    },
    templates::ScaffoldedTemplate,
    utils::{check_no_whitespace, input_no_whitespace, input_yes_or_no},
};

#[derive(Debug, StructOpt)]
/// Scaffold a new, empty web app
pub struct WebApp {
    /// Name of the app to scaffold
    pub name: Option<String>,

    /// Description of the app to scaffold
    pub description: Option<String>,

    #[structopt(long)]
    /// Whether to setup the holonix development environment for this web-app
    pub setup_nix: bool,

    #[structopt(short, long, parse(try_from_str = PackageManager::from_str))]
    /// The package manager to use for the hc-scaffold commands.
    pub package_manager: Option<PackageManager>,

    #[structopt(long = "holo", hidden = true)]
    pub holo_enabled: bool,

    #[structopt(long, short = "F")]
    /// Whether to skip setting up an initial DNA and it's zome(s) after the web app is scaffolded
    pub disable_fast_track: bool,
}

impl WebApp {
    pub async fn run(self, template_file_tree: FileTree) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let name = match self.name {
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

        WebApp::is_valid_template(&template_file_tree)?;

        let setup_nix = if self.setup_nix {
            self.setup_nix
        } else {
            input_yes_or_no(
                "Do you want to set up the holonix development environment for this project?",
                Some(true),
            )?
        };

        let package_manager = match self.package_manager {
            Some(p) => p,
            None => PackageManager::choose()?,
        };

        let ScaffoldedTemplate {
            file_tree,
            next_instructions,
        } = scaffold_web_app(
            &name,
            self.description.as_deref(),
            package_manager,
            !setup_nix,
            &template_file_tree,
            self.holo_enabled,
        )?;

        build_file_tree(file_tree, &app_folder)?;

        if setup_nix {
            if let Err(err) = setup_nix_developer_environment(&app_folder) {
                fs::remove_dir_all(&app_folder).await?;
                return Err(err)?;
            }
        }

        setup_git_environment(&app_folder)?;

        println!("\nYour Web hApp {} has been scaffolded!", name.italic());

        if let Some(i) = next_instructions {
            println!("\n{}", i);
        }

        Ok(())
    }

    fn is_valid_template(template_file_tree: &FileTree) -> ScaffoldResult<()> {
        if file_content(template_file_tree, &PathBuf::from("web-app/README.md.hbs")).is_err() {
            return Err(ScaffoldError::MalformedTemplate(
                "Template does not contain a README.md.hbs file in its \"web-app\" directory"
                    .to_string(),
            ))?;
        }
        Ok(())
    }
}
