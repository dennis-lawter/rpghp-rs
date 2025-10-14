//! Project-wide definitions and utilities.

use std::path::PathBuf;

pub type CrateResult<T> = Result<T, CrateError>;

#[derive(thiserror::Error, Debug)]
pub enum CrateError {
    #[error("{0} not set in .env")]
    EnvVarMissing(String),

    #[error("Poem runtime error: {0}")]
    PoemRuntimeError(std::io::Error),

    #[error("SQLx Error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("SQLx Migration Error: {0}")]
    SqlxMigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Handlebars Error: {0}")]
    HandlebarsTemplateError(#[from] handlebars::TemplateError),

    #[error("Filesystem Error: {0}")]
    FilesystemError(#[from] std::io::Error),

    #[error("Could not strip an expected prefix `{0}` from a directory path `{1}`")]
    PathStripPrefixError(PathBuf, String),
}
