#![allow(clippy::result_large_err)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

mod prelude;
use prelude::*;

mod config;
mod server;

use config::Config;
use server::WebServer;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let cfg = Config::new()?;

    let server = WebServer::new(cfg)?;
    server.serve().await?;

    Ok(())
}
