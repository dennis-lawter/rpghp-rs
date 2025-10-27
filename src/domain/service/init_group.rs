use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::init_group::InitGroupEntity;
use crate::domain::repository::core::RepositoryRegistry;

#[derive(Clone)]
#[allow(dead_code)]
pub struct InitGroupService {
    repos: RepositoryRegistry,
}
impl InitGroupService {
    pub const fn new(repos: RepositoryRegistry) -> Self {
        Self { repos }
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
            .repos
            .session
            .find_by_id_and_secret(&session_id, &secret)
            .await?;
        // TODO: proper tree appending
        let rank = rank.unwrap_or(0);
        let init_group = InitGroupEntity {
            rpghp_init_group_id: Uuid::new_v4(),
            session_id: session.rpghp_session_id,
            rank,
        };

        self.repos.init_group.create(&init_group).await?;

        Ok(init_group)
    }
}
