//! rpghp-rs is a simple webserver.
//! The goal is to provide a visual aid for TTRPGs.
//! Game Masters manage health, status effects, and initiative order.
//! Players can view the updates made by the Game Masters in real time.

mod config;
mod domain;
mod prelude;
mod server;

use config::Config;
use server::WebServer;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let cfg = Config::new()?;

    let server = WebServer::new(cfg);
    server.serve().await?;

    Ok(())
}
