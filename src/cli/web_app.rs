use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use colored::Colorize;
use convert_case::{Case, Casing};
use structopt::StructOpt;
use tokio::fs;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{build_file_tree, load_directory_into_memory, FileTree},
    scaffold::{
        app::{git::setup_git_environment, nix::setup_nix_developer_environment, AppFileTree},
        config::ScaffoldConfig,
        dna::scaffold_dna,
        web_app::{
            package_manager::{PackageManager, SubCommand},
            scaffold_web_app,
            template_type::TemplateType,
        },
        zome::scaffold_zome_pair,
    },
    templates::ScaffoldedTemplate,
    utils::{check_no_whitespace, input_no_whitespace, input_with_case, input_yes_or_no},
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
    pub async fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
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

        let template_file_tree = template_type.file_tree()?;

        template_type.check_valid_template()?;

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
            mut file_tree,
            next_instructions,
        } = scaffold_web_app(
            &name,
            self.description.as_deref(),
            package_manager,
            !setup_nix,
            &template_file_tree,
            self.holo_enabled,
        )?;

        if !template_type.is_nixified_custom_template() {
             ScaffoldConfig::write_to_package_json(&mut file_tree, template_type)?;
        }

        build_file_tree(file_tree, &app_folder)?;

        let mut nix_instructions = "";

        if setup_nix {
            if let Err(err) = setup_nix_developer_environment(&app_folder) {
                fs::remove_dir_all(&app_folder).await?;
                return Err(err)?;
            }
            nix_instructions = "\n  nix develop";
        }

        println!("Your Web hApp {} has been scaffolded!\n", name.italic());

        let mut disable_fast_track = self.disable_fast_track;

        if !disable_fast_track
            && input_yes_or_no("Do you want to scaffold an initial DNA? (y/n)", None)?
        {
            WebApp::scaffold_initial_dna_and_zomes(&name, template_file_tree, &current_dir)?;
        } else {
            disable_fast_track = true;
        }

        setup_git_environment(&app_folder)?;

        if let Some(instructions) = next_instructions {
            println!("{instructions}");
        } else {
            let dna_instructions = disable_fast_track.then_some(
                r#"
- Get your project to compile by adding a DNA and then following the next insturctions to add a zome to that DNA:

  hc scaffold dna"#).unwrap_or_default();
            println!(
                r#"
This skeleton provides the basic structure for your Holochain web application.
The UI is currently empty; you will need to import necessary components into the top-level app component to populate it.

Here's how you can get started with developing your application:

- Set up your development environment:

  cd {name}{nix_instructions}
  {} {dna_instructions}

- Scaffold an entry-type for your hApp:

  hc scaffold entry-type

- Then, at any point in time you can start your application with:

  {}
                "#,
                package_manager.run_command_string(SubCommand::Install, None),
                package_manager.run_command_string(SubCommand::Run("start".to_string()), None)
            );
        }

        Ok(())
    }

    fn scaffold_initial_dna_and_zomes(
        name: &str,
        template_file_tree: FileTree,
        path: &Path,
    ) -> ScaffoldResult<()> {
        env::set_current_dir(PathBuf::from(&name))?;
        let dna_name = input_with_case(
            "Initial DNA name (snake_case):",
            Some(&name.to_case(Case::Snake)),
            Case::Snake,
        )?;

        let file_tree = load_directory_into_memory(&path.join(name))?;
        let app_file_tree = AppFileTree::get_or_choose(file_tree, Some(name))?;

        let ScaffoldedTemplate { file_tree, .. } =
            scaffold_dna(app_file_tree, &template_file_tree, &dna_name)?;

        if input_yes_or_no("Do you want to scaffold an initial coordinator/integrity zome pair for your DNA? (y/n)", None)? {
            scaffold_zome_pair(file_tree, template_file_tree, &dna_name)?;
            println!("Coordinator/integrity zome pair scaffolded.")
        } else {
            build_file_tree(file_tree, ".")?;
            println!("DNA scaffolded.");
        }

        Ok(())
    }
}
