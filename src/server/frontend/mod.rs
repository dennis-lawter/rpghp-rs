use poem::Route;
use poem::handler;
use poem_openapi::payload;

use crate::config::Config;

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

pub fn create_route(_cfg: &Config) -> Route {
    Route::new().nest("/", index)
}
