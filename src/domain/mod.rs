//! Accessor for persistant storage.

pub mod core;
pub mod entity;
mod error;
mod repository;
pub mod service;

pub use core::Domain;

pub use error::DomainError;
pub use error::DomainResult;
