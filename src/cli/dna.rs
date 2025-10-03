use colored::Colorize;
use convert_case::Case;
use structopt::StructOpt;

use crate::{
    file_tree::{build_file_tree, load_directory_into_memory},
    scaffold::{app::AppFileTree, dna::scaffold_dna, web_app::template_type::TemplateType},
    templates::ScaffoldedTemplate,
    utils::{check_case, input_with_case},
};

#[derive(Debug, StructOpt)]
/// Scaffold a DNA into an existing app
pub struct Dna {
    #[structopt(long)]
    /// Name of the app in which you want to scaffold the DNA
    pub app: Option<String>,

    /// Name of the DNA being scaffolded
    pub name: Option<String>,
}

impl Dna {
    pub fn run(self, template_type: &TemplateType) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let file_tree = load_directory_into_memory(&current_dir)?;

        let name = match self.name {
            Some(n) => {
                check_case(&n, "dna name", Case::Snake)?;
                n
            }
            None => input_with_case("DNA name (snake_case):", None, Case::Snake)?,
        };

        let app_file_tree = AppFileTree::get_or_choose(file_tree, self.app.as_deref())?;

        let ScaffoldedTemplate {
            file_tree,
            next_instructions,
        } = scaffold_dna(app_file_tree, &template_type.file_tree()?, &name)?;

        build_file_tree(file_tree, ".")?;

        println!("\nDNA {} scaffolded!", name.italic());

        if let Some(i) = next_instructions {
            println!("\n{i}");
        } else {
            println!(
                r#"
Add new zomes to your DNA with:

  hc scaffold zome
                "#,
            );
        }

        Ok(())
    }
}
