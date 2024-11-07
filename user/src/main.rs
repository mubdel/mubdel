use async_graphql::{EmptySubscription, Result};
use clap::Parser;

use startup::startup::Startup;
use ty::service::Service;

use crate::hander::graphql_handler;
use crate::mutation::RootMutation;
use crate::query::RootQuery;

mod hander;
mod mutation;
mod query;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long)]
    cfg: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let s = Startup::new(
        args.cfg,
        Service::User,
        RootQuery,
        RootMutation,
        EmptySubscription,
        graphql_handler,
    )?;

    s.startup().await?;

    Ok(())
}
