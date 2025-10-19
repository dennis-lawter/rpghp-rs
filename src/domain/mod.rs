//! Accessor for persistant storage.

mod domain_actions;
mod domain_error;
pub mod facade;
pub mod records;

pub use domain_error::DomainError;
pub use domain_error::DomainResult;
pub use facade::Domain;
