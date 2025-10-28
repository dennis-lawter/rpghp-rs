use std::sync::Arc;

use poem::IntoResponse;
use poem::handler;
use poem::web::Data;
use serde_json::json;

use crate::server::application_context::ApplicationContext;

#[handler]
pub fn index(state: Data<&Arc<ApplicationContext>>) -> poem::Result<impl IntoResponse> {
    let data = json!({ "header": "Hello, World!" });
    state.render("index", data)
}
