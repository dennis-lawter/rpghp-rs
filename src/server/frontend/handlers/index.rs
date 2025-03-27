use poem::IntoResponse;
use poem::handler;
use poem::web::Data;
use serde_json::json;

use crate::server::frontend::frontend_shared_state::FrontendSharedState;

#[handler]
pub fn index(state: Data<&FrontendSharedState>) -> poem::Result<impl IntoResponse> {
    let data = json!({ "header": "Hello, World!" });
    state.render("index", data)
}
