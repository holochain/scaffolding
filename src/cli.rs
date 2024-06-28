#![doc = include_str!("../guides/cli.md")]

use crate::error::ScaffoldError;
use crate::file_tree::{load_directory_into_memory, FileTree};
use crate::scaffold::config::ScaffoldConfig;
use crate::scaffold::example::ExampleType;
use crate::scaffold::web_app::uis::UiFramework;

use colored::Colorize;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;

mod collection;
mod dna;
mod entry_type;
mod example;
mod link_type;
mod template;
mod web_app;
mod zome;

#[derive(Debug, StructOpt)]
pub struct HcScaffold {
    #[structopt(short, long)]
    /// The template to use for the hc-scaffold commands
    /// Can either be an option from the built-in templates: "vanilla", "vue", "lit", "svelte", "react", "headless"
    /// Or a path to a custom template
    template: Option<String>,

    #[structopt(subcommand)]
    command: HcScaffoldCommand,
}

/// A command-line interface for creating and modifying a Holochain application (hApp).
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffoldCommand {
    WebApp(web_app::WebApp),
    Template(template::Template),
    Dna(dna::Dna),
    Zome(zome::Zome),
    EntryType(entry_type::EntryType),
    LinkType(link_type::LinkType),
    Collection(collection::Collection),
    Example(example::Example),
}

impl HcScaffold {
    pub async fn run(self) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let scaffold_config = ScaffoldConfig::from_package_json_path(&current_dir)?;
        let (template, template_file_tree) =
            self.get_template(&current_dir, scaffold_config.as_ref())?;

        match self.command {
            HcScaffoldCommand::WebApp(web_app) => web_app.run(template_file_tree).await?,
            HcScaffoldCommand::Template(template) => template.run(template_file_tree)?,
            HcScaffoldCommand::Dna(dna) => dna.run(template_file_tree)?,
            HcScaffoldCommand::Zome(zome) => zome.run(template_file_tree)?,
            HcScaffoldCommand::EntryType(entry_type) => entry_type.run(template_file_tree)?,
            HcScaffoldCommand::LinkType(link_type) => link_type.run(template_file_tree)?,
            HcScaffoldCommand::Collection(collection) => collection.run(template_file_tree)?,
            HcScaffoldCommand::Example(example) => {
                example.run(template_file_tree, &template).await?
            }
        }

        Ok(())
    }

    fn get_template(
        &self,
        current_dir: &Path,
        scaffold_config: Option<&ScaffoldConfig>,
    ) -> Result<(String, FileTree), ScaffoldError> {
        let template = match (scaffold_config, &self.template) {
            (Some(config), Some(template)) if &config.template != template => {
                return Err(ScaffoldError::InvalidArguments(format!(
                    "The value {} passed with `--template` does not match the template the web-app was scaffolded with: {}",
                    template.italic(),
                    config.template.italic(),
                )));
            }
            (Some(config), _) if !Path::new(&config.template).exists() => Some(&config.template),
            (_, t) => t.as_ref(),
        };

        match template {
            Some(template) => match template.to_lowercase().as_str() {
                "lit" | "svelte" | "vanilla" | "vue" | "react" | "headless" => {
                    let ui_framework = UiFramework::from_str(template)?;
                    Ok((ui_framework.name(), ui_framework.template_filetree()?))
                }
                custom_template_path if Path::new(custom_template_path).exists() => {
                    let templates_dir = current_dir.join(custom_template_path);
                    Ok((
                        custom_template_path.to_string(),
                        load_directory_into_memory(&templates_dir)?,
                    ))
                }
                path => Err(ScaffoldError::PathNotFound(PathBuf::from(path))),
            },
            None => {
                let ui_framework = match &self.command {
                    HcScaffoldCommand::WebApp { .. } => UiFramework::choose()?,
                    HcScaffoldCommand::Example(example::Example { ref example, .. }) => {
                        match example {
                            Some(ExampleType::HelloWorld) => UiFramework::Vanilla,
                            _ => UiFramework::choose_non_vanilla()?,
                        }
                    }
                    _ => {
                        let file_tree = load_directory_into_memory(current_dir)?;
                        UiFramework::try_from(&file_tree)?
                    }
                };
                Ok((ui_framework.name(), ui_framework.template_filetree()?))
            }
        }
    }
}
