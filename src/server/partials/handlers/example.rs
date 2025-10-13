use std::sync::Arc;

use poem::IntoResponse;
use poem::handler;
use poem::web::Data;
use serde_json::json;

use crate::server::shared_state::SharedState;

#[handler]
#[allow(clippy::needless_pass_by_value)]
pub fn example(state: Data<&Arc<SharedState>>) -> poem::Result<impl IntoResponse> {
    let data = json!({
        "icon": "/assets/ext/game-icons.net.svg/icons/ffffff/transparent/1x1/lorc/pummeled.svg",
        "name": "Low HP Guy",
        "curr_hp": 8,
        "max_hp": 81,
        "approx_hp": 0.1
    });
    state.render("partials/example", data)
}
