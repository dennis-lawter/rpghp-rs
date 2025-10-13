use std::sync::Arc;

use poem::IntoResponse;
use poem::handler;
use poem::web::Data;
use serde_json::json;

use crate::server::shared_state::SharedState;

#[handler]
#[allow(clippy::needless_pass_by_value)]
pub fn index(state: Data<&Arc<SharedState>>) -> poem::Result<impl IntoResponse> {
    let data = json!({ "header": "Hello, World!" });
    state.render("index", data)
}
