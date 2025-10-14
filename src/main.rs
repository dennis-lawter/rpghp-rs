//! rpghp-rs is a simple webserver.
//! The goal is to provide a visual aid for TTRPGs.
//! Game Masters manage health, status effects, and initiative order.
//! Players can view the updates made by the Game Masters in real time.

use color_eyre::eyre::Result;
use config::Config;
use server::WebServer;

mod config;
mod domain;
mod prelude;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    initialize()?;

    let cfg = Config::new()?;
    let server = WebServer::new(cfg).await?;
    server.serve().await?;

    Ok(())
}

fn initialize() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    Ok(())
}
