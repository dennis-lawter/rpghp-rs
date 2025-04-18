use crate::prelude::*;

// #[derive(Debug)]
// pub enum EnvLevel {
//     Dev,
//     Prod,
// }
// impl EnvLevel {
//     pub fn from_str(input: &str) -> CrateResult<Self> {
//         match input {
//             "dev" => Ok(Self::Dev),
//             "prod" => Ok(Self::Prod),
//             invalid => Err(CrateError::UnrecognizedEnvLvl {
//                 lvl: invalid.to_owned(),
//             }),
//         }
//     }
// }

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    // pub env_lvl: EnvLevel,
    pub base_url: String,
    pub contact_name: String,
    pub contact_email: String,
}
impl Config {
    pub fn new() -> CrateResult<Self> {
        Self::init_dot_env()?;
        // let env_lvl_raw = Self::get_env("ENVIRONMENT_LEVEL")?;
        // let env_lvl = EnvLevel::from_str(&env_lvl_raw)?;
        let db_url = Self::get_env("DATABASE_URL")?;
        let base_url = Self::get_env("BASE_URL")?;
        let contact_name = Self::get_env("CONTACT_NAME")?;
        let contact_email = Self::get_env("CONTACT_EMAIL")?;
        Ok(Self {
            db_url,
            // env_lvl,
            base_url,
            contact_name,
            contact_email,
        })
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
