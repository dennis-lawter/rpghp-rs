use crate::prelude::*;

use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::middleware::AddDataEndpoint;

use super::shared_state::SharedState;

mod handlers;

pub struct Partials;
impl Partials {
    pub async fn create_route(
        state: Arc<SharedState>
    ) -> CrateResult<AddDataEndpoint<Route, Arc<SharedState>>> {
        Ok(Route::new()
            .nest("example", handlers::example::example)
            .data(state))
    }
}
