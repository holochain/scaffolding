use holochain_scaffolding_cli::cli::HcScaffold;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HcScaffold::from_args().run().await
}
