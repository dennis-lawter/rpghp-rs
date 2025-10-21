//! Manages everything required for the webserver.
//! Primarily concerned with data input and output.
//! Utilizes the domain to store and retrieve data.

mod api;
mod core;
mod frontend;
mod partials;
mod shared_state;

pub use core::WebServer;
