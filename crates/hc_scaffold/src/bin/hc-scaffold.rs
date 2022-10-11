use structopt::StructOpt;
use holochain_cli_scaffold::cli::HcScaffold;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  HcScaffold::from_args().run().await
}