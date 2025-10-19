use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::command::Command;
use crate::domain::command::DeleteCommand;
use crate::domain::domain_error::DomainResult;
use crate::domain::repository::session::SessionRepository;

pub struct DeleteSessionCommand {
    id: Uuid,
}
impl DeleteSessionCommand {
    pub fn new(id_str: &str) -> DomainResult<Self> {
        let id = Uuid::try_parse(id_str).map_err(DomainError::InvalidUuid)?;
        Ok(Self { id })
    }
}
impl Command for DeleteSessionCommand {}
impl DeleteCommand<SessionRepository> for DeleteSessionCommand {
    async fn exec(
        &self,
        repo: &SessionRepository,
    ) -> DomainResult<()> {
        repo.delete(&self.id).await
    }
}
