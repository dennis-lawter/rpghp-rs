use crate::prelude::*;

use frontend_shared_state::FrontendSharedState;
use poem::EndpointExt;
use poem::IntoResponse;
use poem::Route;
use poem::handler;
use poem::middleware::AddDataEndpoint;
use poem::web::Data;
use serde_json::json;

mod frontend_shared_state;

#[handler]
fn index(state: Data<&FrontendSharedState>) -> poem::Result<impl IntoResponse> {
    let data = json!({ "header": "Hello, World!" });
    state.render("index", data)
}

pub fn create_route() -> CrateResult<AddDataEndpoint<Route, FrontendSharedState>> {
    let frontend_shared_state = FrontendSharedState::new()?;
    Ok(Route::new().nest("/", index).data(frontend_shared_state))
}
