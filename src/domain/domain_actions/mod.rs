use sqlx::PgPool;

use crate::config::Config;
use crate::prelude::*;

pub mod creature;
pub mod session;

#[derive(Clone)]
pub struct Domain {
    db: PgPool,
}

impl Domain {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let db = Self::get_db_pool(cfg).await?;
        Self::migrate_db(&db).await?;

        Ok(Self { db })
    }

    async fn get_db_pool(cfg: &Config) -> CrateResult<PgPool> {
        sqlx::Pool::<sqlx::Postgres>::connect(&cfg.db_url)
            .await
            .map_err(CrateError::SqlxError)
    }

    async fn migrate_db(pool: &PgPool) -> CrateResult<()> {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .map_err(CrateError::SqlxMigrationError)?;
        Ok(())
    }
}
