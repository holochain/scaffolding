#![doc = include_str!("../guides/cli.md")]

use crate::error::ScaffoldError;
use crate::file_tree::load_directory_into_memory;
use crate::scaffold::config::ScaffoldConfig;
use crate::scaffold::example::ExampleType;
use crate::scaffold::web_app::template_type::TemplateType;

use colored::Colorize;
use std::{path::Path, str::FromStr};
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
    #[structopt(short, long, parse(try_from_str = TemplateType::from_str))]
    /// The template to use for the hc-scaffold commands
    /// Can either be an option from the built-in templates: "vanilla", "vue", "lit", "svelte", "react", "headless"
    /// Or a path to a custom template
    template: Option<TemplateType>,

    #[structopt(long)]
    /// Skip reading from `hcScaffold` configurations. Largely useful for hApps built with
    /// nix-wrapper based custom templates
    skip_config_check: bool,

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
        let scaffold_config = if !self.skip_config_check {
            ScaffoldConfig::from_package_json_path(&current_dir)?
        } else {
            None
        };
        let template_type = self.get_template_type(&current_dir, scaffold_config.as_ref())?;

        match self.command {
            HcScaffoldCommand::WebApp(web_app) => web_app.run(&template_type).await,
            HcScaffoldCommand::Template(template) => template.run(&template_type),
            HcScaffoldCommand::Dna(dna) => dna.run(&template_type),
            HcScaffoldCommand::Zome(zome) => zome.run(&template_type),
            HcScaffoldCommand::EntryType(entry_type) => entry_type.run(&template_type),
            HcScaffoldCommand::LinkType(link_type) => link_type.run(&template_type),
            HcScaffoldCommand::Collection(collection) => collection.run(&template_type),
            HcScaffoldCommand::Example(example) => example.run(&template_type).await,
        }
    }

    fn get_template_type(
        &self,
        current_dir: &Path,
        scaffold_config: Option<&ScaffoldConfig>,
    ) -> Result<TemplateType, ScaffoldError> {
        // Read template_type config if no `--template` flag is provided and use it or
        // ensure that if a `--template` is explicity provided, it matches the original
        // template the app was scaffolded with
        let template = match (scaffold_config, &self.template) {
            (Some(config), Some(template)) if config.template != *template => {
                return Err(ScaffoldError::InvalidArguments(format!(
                    "The value {} passed with `--template` does not match the template the web-app was scaffolded with: {}",
                    template.name().italic(),
                    config.template.name().italic(),
                )));
            }
            (Some(config), _) => Some(&config.template),
            (_, t) => t.as_ref(),
        };

        match template {
            Some(template) => Ok(template.clone()),
            None => {
                let template_type = match &self.command {
                    HcScaffoldCommand::WebApp { .. } => TemplateType::choose()?,
                    HcScaffoldCommand::Example(example::Example { ref example, .. }) => {
                        match example {
                            Some(ExampleType::HelloWorld) => TemplateType::Vanilla,
                            Some(ExampleType::Forum) => TemplateType::choose_non_vanilla()?,
                            None => TemplateType::choose_non_headless()?,
                        }
                    }
                    _ => TemplateType::try_from(&load_directory_into_memory(current_dir)?)?,
                };
                Ok(template_type)
            }
        }
    }
}
