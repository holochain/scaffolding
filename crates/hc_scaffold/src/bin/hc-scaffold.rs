use structopt::StructOpt;
use holochain_scaffolding_cli::cli::HcScaffold;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  HcScaffold::from_args().run().await
}