use std::sync::Arc;

use poem::Route;
use poem::Server;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;

use super::api::Api;
use super::frontend::Frontend;
use super::partials::Partials;
use super::shared_state::SharedState;
use crate::config::Config;
use crate::prelude::*;

pub struct WebServer {
    cfg: Config,
}
impl WebServer {
    pub const fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub async fn serve(self) -> CrateResult<()> {
        let shared_state = SharedState::new(&self.cfg).await?;
        let shared_state_arc = Arc::new(shared_state);

        let assets_routes = StaticFilesEndpoint::new("./assets");
        let api_routes = Api::create_route(&self.cfg, shared_state_arc.clone());
        let frontend_routes = Frontend::create_route(shared_state_arc.clone());
        let partials_routes = Partials::create_route(shared_state_arc.clone());

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
