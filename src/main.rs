#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;

mod app_state;
mod config;
mod prelude;
mod records;

use crate::prelude::*;

use app_state::AppState;
use config::Config;

#[get("/hello/{name}")]
async fn greet(state: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    if let Ok(result) = sqlx::query!("select count(*) as cnt from rpghp_session")
        .fetch_one(&state.pool)
        .await
    {
        let cnt = result.cnt.unwrap_or_default();
        format!("Hello {name}! There are {cnt} sessions!")
    } else {
        "Query error".to_owned()
    }
}

#[tokio::main]
async fn main() -> CrateResult<()> {
    let cfg = Config::new()?;
    let state = AppState::new(&cfg).await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            // endpoints
            .service(greet)
    })
    .bind(("0.0.0.0", 8080))
    .map_err(CrateError::ActixBindError)?
    .run()
    .await
    .map_err(CrateError::ActixRuntimeError)
}
