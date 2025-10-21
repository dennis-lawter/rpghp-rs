//! Project-wide definitions and utilities.

pub type CrateResult<T> = Result<T, CrateError>;

#[derive(thiserror::Error, Debug)]
pub enum CrateError {
    #[error("{0} not set in .env")]
    EnvVarMissing(String),

    #[error("Poem runtime error: {0}")]
    PoemRuntimeError(std::io::Error),

    #[error("SQLx Error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Handlebars Error: {0}")]
    HandlebarsTemplateError(#[from] handlebars::TemplateError),

    #[error("Filesystem Error: {0}")]
    FilesystemError(#[from] std::io::Error),

    #[error("Could not strip an expected prefix `{0}` from a directory path `{1}`")]
    PathStripPrefixError(String, String),
}
