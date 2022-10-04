use structopt::StructOpt;
use crate::{error::ScaffoldResult, app};

/// The list of subcommands for `hc sandbox`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcScaffold {
    /// Scaffold a new web app
    WebApp {
        /// Name of the app to scaffold
        app_name: String,
    },
}

impl HcScaffold {
  pub fn run(self) -> anyhow::Result<()> {
    match self {
      HcScaffold::WebApp {
        app_name
      } => app::scaffold_web_app(app_name)?
    }
    
    Ok(())
  }
}