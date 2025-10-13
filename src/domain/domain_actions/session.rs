use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::records::Record;
use crate::domain::records::session::SessionRecord;

use super::Domain;

impl Domain {
    pub async fn create_session(&self) -> DomainResult<SessionRecord> {
        let session_record = SessionRecord::new();
        session_record.save(&self.db).await?;
        Ok(session_record)
    }

    pub async fn get_session(
        &self,
        id: &str,
    ) -> DomainResult<SessionRecord> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        SessionRecord::find_by_id(&self.db, &id).await
    }

    pub async fn delete_session(
        &self,
        id: &str,
        secret: &str,
    ) -> DomainResult<()> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let session = SessionRecord::find_by_id_and_secret(&self.db, &id, &secret).await?;
        session.delete(&self.db).await
    }
}
