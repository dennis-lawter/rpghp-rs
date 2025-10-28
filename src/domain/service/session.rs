use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::session::SessionEntity;
use crate::domain::repository::core::RepositoryContext;

#[derive(Clone)]
pub struct SessionService {
    repos: RepositoryContext,
}
impl SessionService {
    pub const fn new(repos: RepositoryContext) -> Self {
        Self { repos }
    }

    pub async fn create_session(&self) -> DomainResult<SessionEntity> {
        let session_entity = SessionEntity::new();
        self.repos.session.create(&session_entity).await?;
        Ok(session_entity)
    }

    pub async fn get_session(
        &self,
        session_id: &str,
    ) -> DomainResult<SessionEntity> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        self.repos.session.find_by_id(&session_id).await
    }

    pub async fn delete_session(
        &self,
        session_id: &str,
        secret: &str,
    ) -> DomainResult<()> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let session = self
            .repos
            .session
            .find_by_id_and_secret(&session_id, &secret)
            .await?;
        self.repos.session.delete(&session).await?;
        Ok(())
    }
}
