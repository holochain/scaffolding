use colored::Colorize;
use holochain_scaffolding_cli::cli::HcScaffold;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = HcScaffold::from_args().run().await {
        println!("{}", e.to_string().red());
    }
    Ok(())
}
