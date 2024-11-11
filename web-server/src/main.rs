use std::env::var;

use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};

use crate::spa::SPAEndpoint;

pub mod spa;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let app = Route::new().nest(
        "/",
        SPAEndpoint::new(var("WEB_CLIENT_PATH")?, "index.html").with_assets("assets"),
    );

    Server::new(TcpListener::bind("0.0.0.0:4005"))
        .run(app)
        .await?;

    Ok(())
}
