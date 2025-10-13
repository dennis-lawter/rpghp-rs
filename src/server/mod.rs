//! Manages everything required for the webserver.
//! Primarily concerned with data input and output.
//! Utilizes the domain to store and retrieve data.

mod api;
mod frontend;
mod partials;
mod shared_state;
mod web_server;

pub use web_server::WebServer;
