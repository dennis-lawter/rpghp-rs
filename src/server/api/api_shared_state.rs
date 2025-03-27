use crate::prelude::*;

use sqlx::PgPool;

use crate::Config;

#[derive(Clone)]
pub struct ApiSharedState {
    pub pool: PgPool,
}
impl ApiSharedState {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let pool = Self::get_pool(cfg).await?;
        Self::migrate_db(&pool).await?;
        Ok(Self { pool })
    }

    async fn get_pool(cfg: &Config) -> CrateResult<PgPool> {
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
