use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::Server;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::Tracing;

use super::api::Api;
use super::application_context::ApplicationContext;
use super::frontend::Frontend;
use super::partials::Partials;
use crate::config::Config;
use crate::prelude::*;

pub struct WebServer {
    cfg: Config,
    state: Arc<ApplicationContext>,
}
impl WebServer {
    pub async fn new(cfg: Config) -> CrateResult<Self> {
        let state = Arc::new(ApplicationContext::new(&cfg).await?);
        Ok(Self { cfg, state })
    }

    pub async fn serve(self) -> CrateResult<()> {
        let routes = self.create_root_route();
        let routes = routes.with(Tracing);

        Server::new(TcpListener::bind(self.cfg.base_url.clone()))
            .run(routes)
            .await
            .map_err(CrateError::PoemRuntimeError)
    }

    fn create_root_route(&self) -> Route {
        Route::new()
            .nest("/assets", Self::assets_route())
            .nest("/api", Api::create_route(&self.cfg, self.state.clone()))
            .nest("/partials", Partials::create_route(self.state.clone()))
            .nest("/", Frontend::create_route(self.state.clone()))
    }

    fn assets_route() -> StaticFilesEndpoint {
        StaticFilesEndpoint::new("./assets")
    }
}
