//! Configuration struct for loading required ENV VARs

#[allow(unused_imports)]
use crate::prelude::*;

/// Contains necessary configurations for the site to function.
/// Values are loaded via the [dotenv] crate.
#[derive(Debug)]
pub struct Config {
    /// Full URL to the database
    pub db_url: String,

    /// Hostname to the webserver
    pub base_url: String,

    /// A name used in documentation for who to contact for support
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
        Self::init_dot_env()?;
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

    /// Initializes the [dotenv] crate.
    ///
    /// Dotenv errors are cast to `CrateError::DotEnvInstallError`.
    fn init_dot_env() -> CrateResult<()> {
        dotenv::dotenv().map_err(|_| CrateError::DotEnvInstallError)?;
        Ok(())
    }

    /// Loads an ENV VAR matching the given name.
    ///
    /// Returns `Err(CrateError::EnvMissing)` if the required ENV VAR is not defined.
    fn get_env(var: &str) -> CrateResult<String> {
        std::env::var(var).map_err(|_| CrateError::EnvMissing {
            var: var.to_owned(),
        })
    }
}
