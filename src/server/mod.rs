//! Manages everything required for the webserver.
//! Primarily concerned with data input and output.
//! Utilizes the domain to store and retrieve data.

mod api;
mod application_context;
mod core;
mod frontend;
mod partials;

pub use core::WebServer;
