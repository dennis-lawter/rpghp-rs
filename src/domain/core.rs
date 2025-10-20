use sqlx::PgPool;

use crate::config::Config;
use crate::domain::service::creature::CreatureService;
use crate::domain::service::session::SessionService;
use crate::prelude::*;

#[derive(Clone)]
pub struct Domain {
    pub session_service: SessionService,
    pub creature_service: CreatureService,
}

impl Domain {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let db = Self::get_db_pool(cfg).await?;

        let session_service = SessionService::new(db.clone());
        let creature_service = CreatureService::new(db.clone());

        Ok(Self {
            session_service,
            creature_service,
        })
    }

    async fn get_db_pool(cfg: &Config) -> CrateResult<PgPool> {
        sqlx::Pool::<sqlx::Postgres>::connect(&cfg.db_url)
            .await
            .map_err(CrateError::SqlxError)
    }
}
