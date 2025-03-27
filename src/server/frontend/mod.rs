use frontend_shared_state::FrontendSharedState;
use poem::EndpointExt;
use poem::Route;
use poem::handler;
use poem::middleware::AddDataEndpoint;
use poem_openapi::payload;

mod frontend_shared_state;

#[handler]
fn index() -> payload::Html<String> {
    let idx_html = r#"
<link rel="stylesheet" href="/assets/style.css">
<h1>
    Hello, World!
</h1>
"#;
    payload::Html(idx_html.to_owned())
}

pub fn create_route() -> AddDataEndpoint<Route, FrontendSharedState<'static>> {
    let frontend_shared_state = FrontendSharedState::new();
    Route::new().nest("/", index).data(frontend_shared_state)
}
