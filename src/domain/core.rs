use sqlx::PgPool;

use crate::config::Config;
use crate::domain::repository::core::RepositoryContext;
use crate::domain::service::creature::CreatureService;
use crate::domain::service::init_group::InitGroupService;
use crate::domain::service::session::SessionService;
use crate::prelude::*;

#[derive(Clone)]
pub struct ServiceContext {
    pub session: SessionService,
    pub creature: CreatureService,
    pub init_group: InitGroupService,
}

impl ServiceContext {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let db = Self::create_db_connection(cfg).await?;

        let repos = RepositoryContext::new(db);

        let session_service = SessionService::new(repos.clone());
        let creature_service = CreatureService::new(repos.clone());
        let init_group_service = InitGroupService::new(repos.clone());

        Ok(Self {
            session: session_service,
            creature: creature_service,
            init_group: init_group_service,
        })
    }

    async fn create_db_connection(cfg: &Config) -> CrateResult<PgPool> {
        sqlx::Pool::<sqlx::Postgres>::connect(&cfg.db_url)
            .await
            .map_err(CrateError::SqlxError)
    }
}
