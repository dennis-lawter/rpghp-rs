mod handler;

use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::get;
use poem::middleware::AddDataEndpoint;

use super::shared_state::SharedState;

pub struct Frontend;
impl Frontend {
    pub fn create_route(state: Arc<SharedState>) -> AddDataEndpoint<Route, Arc<SharedState>> {
        Route::new()
            .nest("/", get(handler::index::index))
            .data(state)
    }
}
