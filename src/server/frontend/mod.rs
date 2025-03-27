use poem::Route;
use poem::handler;
use poem_openapi::payload;

use crate::config::Config;

#[handler]
fn index() -> payload::Html<String> {
    payload::Html("<h1>Hello, World!</h1>".to_owned())
}

pub fn create_route(_cfg: &Config) -> Route {
    Route::new().nest("/", index)
}
