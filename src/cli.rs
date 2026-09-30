#![doc = include_str!("../guides/cli.md")]

use crate::error::ScaffoldError;
use crate::file_tree::load_directory_into_memory;
use crate::scaffold::config::ScaffoldConfig;
use crate::scaffold::web_app::template_type::TemplateType;

use clap::Parser;
use colored::Colorize;
use std::{path::Path, str::FromStr};

mod collection;
mod dna;
mod entry_type;
mod example;
mod link_type;
mod template;
mod web_app;
mod zome;

#[derive(Debug, Parser)]
#[command(version, propagate_version = true)]
pub struct HcScaffold {
    #[arg(short, long, value_parser = TemplateType::from_str)]
    /// The template to use for the hc-scaffold commands.
    /// Can either be an option from the built-in templates: "svelte", "headless",
    /// or a path to a custom template.
    template: Option<TemplateType>,

    #[command(subcommand)]
    command: HcScaffoldCommand,
}

/// A command-line interface for creating and modifying a Holochain application (hApp).
#[derive(Debug, clap::Subcommand)]
#[command(infer_subcommands = true)]
pub enum HcScaffoldCommand {
    WebApp(web_app::WebApp),
    Template {
        #[command(subcommand)]
        command: template::Template,
    },
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
        let template_type = self.get_template_type(&current_dir, scaffold_config.as_ref())?;

        match self.command {
            HcScaffoldCommand::WebApp(web_app) => web_app.run(&template_type).await,
            HcScaffoldCommand::Template { command } => command.run(&template_type),
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
                    HcScaffoldCommand::Example(example::Example { .. }) => TemplateType::Svelte,
                    _ => TemplateType::try_from(&load_directory_into_memory(current_dir)?)?,
                };
                Ok(template_type)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::{error::ErrorKind, Parser};

    use super::{HcScaffold, HcScaffoldCommand};

    #[test]
    fn entry_type_accepts_space_separated_fields() {
        let cli = HcScaffold::try_parse_from([
            "hc-scaffold",
            "entry-type",
            "post",
            "--fields",
            "title:String",
            "body:String",
        ])
        .expect("space-separated fields should parse");

        let HcScaffoldCommand::EntryType(entry_type) = cli.command else {
            panic!("entry-type arguments should produce the entry-type command");
        };
        let fields = entry_type.fields.expect("fields should be present");

        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn version_is_available_to_all_subcommands() {
        let cases: &[&[&str]] = &[
            &["hc-scaffold", "web-app", "--version"],
            &["hc-scaffold", "template", "--version"],
            &["hc-scaffold", "template", "new", "--version"],
            &["hc-scaffold", "dna", "--version"],
            &["hc-scaffold", "zome", "--version"],
            &["hc-scaffold", "entry-type", "--version"],
            &["hc-scaffold", "link-type", "--version"],
            &["hc-scaffold", "collection", "--version"],
            &["hc-scaffold", "example", "--version"],
        ];

        for args in cases {
            let error = HcScaffold::try_parse_from(*args)
                .expect_err("version requests should stop argument parsing");

            assert_eq!(error.kind(), ErrorKind::DisplayVersion, "{args:?}");
        }
    }
}
