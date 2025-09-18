#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::Arc;

use super::api::Api;
use super::frontend::Frontend;
use super::partials::Partials;
use super::shared_state::SharedState;
use poem::Route;
use poem::Server;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;

use crate::config::Config;

pub struct WebServer {
    cfg: Config,
}
impl WebServer {
    pub const fn new(cfg: Config) -> CrateResult<Self> {
        Ok(Self { cfg })
    }

    pub async fn serve(self) -> CrateResult<()> {
        let assets_routes = StaticFilesEndpoint::new("./assets");

        let shared_state = SharedState::new(&self.cfg).await?;
        let shared_state_rc = Arc::new(shared_state);

        let api_routes = Api::create_route(&self.cfg, shared_state_rc.clone()).await?;
        let frontend_routes = Frontend::create_route(shared_state_rc.clone())?;
        let partials_routes = Partials::create_route(shared_state_rc.clone()).await?;

        let full_routing = Route::new()
            .nest("/assets", assets_routes)
            .nest("/api", api_routes)
            .nest("/partials", partials_routes)
            .nest("/", frontend_routes);

        Server::new(TcpListener::bind(self.cfg.base_url.clone()))
            .run(full_routing)
            .await
            .map_err(CrateError::PoemRuntimeError)
    }
}
