//! Accessor for persistant storage.

mod domain_error;
pub mod facade;
pub mod records;
pub mod service;

pub use domain_error::DomainError;
pub use domain_error::DomainResult;
pub use facade::Domain;
