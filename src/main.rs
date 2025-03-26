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
async fn main() -> CrateResult<()> {
    color_eyre::install()?;
    let cfg = Config::new()?;

    let server = WebServer::new(cfg)?;
    server.serve().await?;

    Ok(())
}
