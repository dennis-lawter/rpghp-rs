use api::Api;
use api::shared_state::ApiSharedState;
use poem::EndpointExt;
use poem::Server;
use poem::listener::TcpListener;

use crate::prelude::*;

use crate::config::Config;

pub mod api;

pub struct WebServer {
    cfg: Config,
}
impl WebServer {
    pub fn new(cfg: Config) -> CrateResult<Self> {
        Ok(Self { cfg })
    }

    pub async fn serve(self) -> CrateResult<()> {
        let api_app_data = ApiSharedState::new(&self.cfg).await?;
        let base_route = Api::create_route().data(api_app_data);
        Server::new(TcpListener::bind(self.cfg.base_url.clone()))
            .run(base_route)
            .await
            .map_err(CrateError::PoemRuntimeError)
    }
}
