mod handlers;

use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::get;
use poem::middleware::AddDataEndpoint;

use super::application_context::ApplicationContext;

pub struct Frontend;
impl Frontend {
    pub fn create_route(
        state: Arc<ApplicationContext>
    ) -> AddDataEndpoint<Route, Arc<ApplicationContext>> {
        Route::new()
            .nest("/", get(handlers::index::index))
            .data(state)
    }
}
