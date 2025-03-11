use crate::prelude::*;

#[derive(Debug)]
pub(crate) enum EnvLevel {
    Dev,
    Prod,
}
impl EnvLevel {
    pub fn from_str(input: &str) -> CrateResult<Self> {
        match input {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            invalid => Err(CrateError::UnrecognizedEnvLvl {
                lvl: invalid.to_owned(),
            }),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Config {
    #[allow(dead_code)]
    pub db_url: String,
    #[allow(dead_code)]
    pub env_lvl: EnvLevel,
}
impl Config {
    pub fn new() -> CrateResult<Self> {
        Self::init_dot_env()?;
        let env_lvl_raw = Self::get_env("ENVIRONMENT_LEVEL")?;
        let env_lvl = EnvLevel::from_str(&env_lvl_raw)?;
        let db_url = Self::get_env("DATABASE_URL")?;
        Ok(Self { db_url, env_lvl })
    }

    fn init_dot_env() -> CrateResult<()> {
        dotenv::dotenv().map_err(|_| CrateError::DotEnvInstallError)?;
        Ok(())
    }

    fn get_env(var: &str) -> CrateResult<String> {
        std::env::var(var).map_err(|_| CrateError::EnvMissing {
            var: var.to_owned(),
        })
    }
}
