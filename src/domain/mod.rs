//! The sole access point to the database.

pub mod command;
mod domain_error;
pub mod entity;
mod facade;
mod repository;
mod service;

pub use domain_error::DomainError;
pub use facade::Domain;
