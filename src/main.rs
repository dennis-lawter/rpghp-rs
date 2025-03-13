#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;

mod app_state;
mod config;
mod prelude;
mod records;
mod routes;

use crate::prelude::*;

use app_state::AppState;
use config::Config;

#[tokio::main]
async fn main() -> CrateResult<()> {
    let cfg = Config::new()?;
    let state = AppState::new(&cfg).await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(routes::session::make_session_resource())
    })
    .bind(("0.0.0.0", 8080))
    .map_err(CrateError::ActixBindError)?
    .run()
    .await
    .map_err(CrateError::ActixRuntimeError)
}
