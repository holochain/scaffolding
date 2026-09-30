use clap::Parser;
use colored::Colorize;
use holochain_scaffolding_cli::cli::HcScaffold;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = HcScaffold::parse().run().await {
        eprintln!("{}", e.to_string().red());
    }
    Ok(())
}
