use crate::prelude::*;

use frontend_shared_state::FrontendSharedState;
use poem::EndpointExt;
use poem::Route;
use poem::get;
use poem::middleware::AddDataEndpoint;

mod frontend_shared_state;
mod handlers;

pub fn create_route() -> CrateResult<AddDataEndpoint<Route, FrontendSharedState>> {
    let frontend_shared_state = FrontendSharedState::new()?;
    Ok(Route::new()
        .nest("/", get(handlers::index::index))
        .data(frontend_shared_state))
}
