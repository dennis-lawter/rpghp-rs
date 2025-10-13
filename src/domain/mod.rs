//! The sole access point to the database.

mod domain_actions;
pub mod domain_error;
pub mod records;

pub use domain_actions::Domain;
pub use domain_error::DomainError;
pub use domain_error::DomainResult;
