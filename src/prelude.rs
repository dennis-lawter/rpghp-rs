pub(crate) type CrateResult<T> = Result<T, CrateError>;

#[derive(thiserror::Error, Debug)]
pub(crate) enum CrateError {
    /// Initialization errors
    #[error("DotEnv install failed")]
    DotEnvInstallError,

    /// Configuration errors
    #[error("{var} not set in .env")]
    EnvMissing { var: String },
    #[error("Unrecognized environment level: {lvl}")]
    UnrecognizedEnvLvl { lvl: String },

    /// Runtime framework errors
    #[error("An error occured while running the actix server: {0}")]
    ActixRuntimeError(std::io::Error),
    #[error("An error occured while binding the port: {0}")]
    ActixBindError(std::io::Error),

    /// SQLx errors
    #[error("SQLx Connect Error: {0}")]
    SqlxConnectError(sqlx::Error),
    #[error("SQLx Migration Error: {0}")]
    SqlxMigrationError(sqlx::migrate::MigrateError),
}
