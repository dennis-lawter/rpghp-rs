use api::Api;
use poem::Route;
use poem::Server;
use poem::listener::TcpListener;

use crate::prelude::*;

use crate::config::Config;

mod api;
mod frontend;

pub struct WebServer {
    cfg: Config,
}
impl WebServer {
    pub fn new(cfg: Config) -> CrateResult<Self> {
        Ok(Self { cfg })
    }

    pub async fn serve(self) -> CrateResult<()> {
        let api_route = Api::create_route(&self.cfg).await?;

        let full_routing = Route::new().nest("/", api_route);

        Server::new(TcpListener::bind(self.cfg.base_url.clone()))
            .run(full_routing)
            .await
            .map_err(CrateError::PoemRuntimeError)
    }
}
