pub type CrateResult<T> = Result<T, CrateError>;

#[derive(thiserror::Error, Debug)]
pub enum CrateError {
    /// Initialization errors
    #[error("DotEnv install failed")]
    DotEnvInstallError,
    #[error("ColorEyre install failed")]
    ColorEyreInstallError(#[from] color_eyre::eyre::ErrReport),

    /// Configuration errors
    #[error("{var} not set in .env")]
    EnvMissing { var: String },
    #[error("Unrecognized environment level: {lvl}")]
    UnrecognizedEnvLvl { lvl: String },

    /// Runtime framework errors
    #[error("Poem runtime error: {0}")]
    PoemRuntimeError(std::io::Error),

    /// SQLx errors
    #[error("SQLx Error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("SQLx Migration Error: {0}")]
    SqlxMigrationError(#[from] sqlx::migrate::MigrateError),

    /// Handlebars errors
    #[error("Handlebars Error: {0}")]
    HandlebarsTemplateError(#[from] handlebars::TemplateError),
}
