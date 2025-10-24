use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::init_group::InitGroupEntity;
use crate::domain::repository::creature::CreatureRepository;
use crate::domain::repository::init_group::InitGroupRepository;
use crate::domain::repository::session::SessionRepository;

#[derive(Clone)]
#[allow(dead_code)]
pub struct InitGroupService {
    init_group_repo: InitGroupRepository,
    creature_repo: CreatureRepository,
    session_repo: SessionRepository,
}
impl InitGroupService {
    pub fn new(db: PgPool) -> Self {
        let init_group_repo = InitGroupRepository::new(db.clone());
        let creature_repo = CreatureRepository::new(db.clone());
        let session_repo = SessionRepository::new(db);
        Self {
            init_group_repo,
            creature_repo,
            session_repo,
        }
    }

    pub async fn create_init_group(
        &self,
        session_id: &str,
        secret: &str,
        rank: Option<i64>,
    ) -> DomainResult<InitGroupEntity> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let session = self
            .session_repo
            .find_by_id_and_secret(&session_id, &secret)
            .await?;
        // TODO: proper tree appending
        let rank = rank.unwrap_or(0);
        let init_group = InitGroupEntity {
            rpghp_init_group_id: Uuid::new_v4(),
            session_id: session.rpghp_session_id,
            rank,
        };

        self.init_group_repo.create(&init_group).await?;

        Ok(init_group)
    }
}
