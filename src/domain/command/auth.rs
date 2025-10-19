use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::domain_error::DomainResult;

#[derive(Clone)]
pub struct DomainAuth {
    session_id: Uuid,
    session_secret: Uuid,
}
impl DomainAuth {
    pub fn new(
        id: &str,
        secret: &str,
    ) -> DomainResult<Self> {
        let session_id = Uuid::try_parse(id).map_err(DomainError::InvalidUuid)?;
        let session_secret = Uuid::try_parse(secret).map_err(DomainError::InvalidUuid)?;
        Ok(Self {
            session_id,
            session_secret,
        })
    }
}
