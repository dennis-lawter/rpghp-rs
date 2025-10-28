use std::sync::Arc;

use poem::EndpointExt;
use poem::Route;
use poem::middleware::AddDataEndpoint;

use super::application_context::ApplicationContext;

mod handlers;

pub struct Partials;
impl Partials {
    pub fn create_route(
        state: Arc<ApplicationContext>
    ) -> AddDataEndpoint<Route, Arc<ApplicationContext>> {
        Route::new()
            .nest("example", handlers::example::example)
            .data(state)
    }
}
