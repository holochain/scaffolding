use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::{dir, file, Build, MergeableFileSystemTree};
use structopt::StructOpt;

use crate::{scaffold::web_app::template_type::TemplateType, utils::input_with_case};

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
/// Manage custom templates
pub enum Template {
    /// Create a new template from an existing scaffolding template
    New,
    /// Clone the template in use into a new custom template
    Clone {
        #[structopt(long)]
        /// The folder to initialize the template into, will end up at "<TO TEMPLATE>"
        to_template: Option<String>,
    },
}

impl Template {
    pub fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        match self {
            Template::New => Template::new_template(template_type),
            Template::Clone { to_template } => Template::clone_template(to_template, template_type),
        }
    }

    fn new_template(from_template: &TemplateType) -> anyhow::Result<()> {
        let name = input_with_case(
            "Enter new template name (kebab-case):",
            Some(&from_template.name()),
            convert_case::Case::Kebab,
        )?;

        let template_file_tree = dir! {
            name.clone() => dir!{
                "template" =>  from_template.file_tree()?,
                "README.md" => file!(include_str!("custom-template/README.md")),
                "flake.nix" => file!(include_str!("custom-template/flake.nix")),
                "run_test.sh" => file!(include_str!("custom-template/run_test.sh"))
            },
        };

        let file_tree = MergeableFileSystemTree::<OsString, String>::from(template_file_tree);

        file_tree.build(&PathBuf::from("."))?;

        println!(r#"Template initialized in path: ./{} "#, name);

        Ok(())
    }

    fn clone_template(
        to_template: Option<String>,
        template_type: &TemplateType,
    ) -> anyhow::Result<()> {
        let target_template = match to_template {
            Some(t) => t,
            None => input_with_case("Enter new template name:", None, convert_case::Case::Kebab)?,
        };

        let template_file_tree = dir! {
            target_template.clone() => template_type.file_tree()?
        };

        let file_tree = MergeableFileSystemTree::<OsString, String>::from(template_file_tree);

        file_tree.build(&PathBuf::from("."))?;

        println!(r#"Template initialized in path: ./{} "#, target_template);

        Ok(())
    }
}
