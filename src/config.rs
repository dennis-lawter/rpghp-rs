//! Module for formalizing and loading the expected environment variables
//! required to run the webserver.

#[allow(unused_imports)]
use crate::prelude::*;

/// Contains every necessary configuration for the site to function.
/// Values are loaded from current environment variables (ENV VAFs),
/// or loaded from the `.env` file in the project's root directory.
#[derive(Debug)]
pub struct Config {
    /// Full URL to the database
    pub db_url: String,
    /// Hostname that the webserver will be served from
    pub base_url: String,
    /// The name used in documentation for who to contact for support
    pub contact_name: String,
    /// An email used in documentation for who to contact for support
    pub contact_email: String,
}
impl Config {
    /// Create a new `Config`
    ///
    /// Returns `Err(CrateError::DotEnvInstallError)` if the [dotenv] crate fails to load.
    /// Returns `Err(CrateError::EnvMissing)` if a required ENV VAR is not defined.
    pub fn new() -> CrateResult<Self> {
        let db_url = Self::get_env("DATABASE_URL")?;
        let base_url = Self::get_env("BASE_URL")?;
        let contact_name = Self::get_env("CONTACT_NAME")?;
        let contact_email = Self::get_env("CONTACT_EMAIL")?;
        Ok(Self {
            db_url,
            base_url,
            contact_name,
            contact_email,
        })
    }

    /// Loads an ENV VAR matching the given name.
    ///
    /// Returns `Err(CrateError::EnvMissing)` if the required ENV VAR is not defined.
    fn get_env(var: &str) -> CrateResult<String> {
        std::env::var(var).map_err(|_| CrateError::EnvVarMissing(var.to_owned()))
    }
}
