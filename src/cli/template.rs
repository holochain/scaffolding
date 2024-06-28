use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::{dir, Build, MergeableFileSystemTree};
use dialoguer::{theme::ColorfulTheme, Input};
use structopt::StructOpt;

use crate::file_tree::FileTree;

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
/// Manage custom templates
pub enum Template {
    /// Clone the template in use into a new custom template
    Clone {
        #[structopt(long)]
        /// The folder to initialize the template into, will end up at "<TO TEMPLATE>"
        to_template: Option<String>,
    },
}

impl Template {
    pub fn run(self, template_file_tree: FileTree) -> anyhow::Result<()> {
        let target_template = match self.target_template() {
            Some(t) => t,
            None => {
                // Enter template name
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter new template name:")
                    .interact()?
            }
        };

        let template_file_tree = dir! {
            target_template.clone() => template_file_tree
        };

        let file_tree = MergeableFileSystemTree::<OsString, String>::from(template_file_tree);

        file_tree.build(&PathBuf::from("."))?;

        match self {
            Template::Clone { .. } => {
                println!(r#"Template initialized to folder {:?} "#, target_template);
            }
        }
        Ok(())
    }

    pub fn target_template(&self) -> Option<String> {
        match self {
            Template::Clone {
                to_template: target_template,
                ..
            } => target_template.clone(),
        }
    }
}
