#[allow(unused_imports)]
use crate::prelude::*;

mod handlers;

use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::get;
use poem::middleware::AddDataEndpoint;

use super::shared_state::SharedState;

pub struct Frontend;
impl Frontend {
    pub fn create_route(
        state: Arc<SharedState>
    ) -> CrateResult<AddDataEndpoint<Route, Arc<SharedState>>> {
        Ok(Route::new()
            .nest("/", get(handlers::index::index))
            .data(state))
    }
}
