use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::session::SessionEntity;
use crate::domain::repository::session::SessionRepository;

#[derive(Clone)]
pub struct SessionService {
    session_repo: SessionRepository,
}
impl SessionService {
    pub const fn new(db: PgPool) -> Self {
        let session_repo = SessionRepository::new(db);
        Self { session_repo }
    }

    pub async fn create_session(&self) -> DomainResult<SessionEntity> {
        let session_entity = SessionEntity::new();
        self.session_repo.create(&session_entity).await?;
        Ok(session_entity)
    }

    pub async fn get_session(
        &self,
        id: &str,
    ) -> DomainResult<SessionEntity> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        self.session_repo.find_by_id(&id).await
    }

    pub async fn delete_session(
        &self,
        id: &str,
        secret: &str,
    ) -> DomainResult<()> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let session = self
            .session_repo
            .find_by_id_and_secret(&id, &secret)
            .await?;
        self.session_repo.delete(&session).await?;
        Ok(())
    }
}
