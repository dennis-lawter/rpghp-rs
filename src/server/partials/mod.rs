use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::middleware::AddDataEndpoint;

use super::shared_state::SharedState;

mod handler;

pub struct Partials;
impl Partials {
    pub fn create_route(state: Arc<SharedState>) -> AddDataEndpoint<Route, Arc<SharedState>> {
        Route::new()
            .nest("example", handler::example::example)
            .data(state)
    }
}
