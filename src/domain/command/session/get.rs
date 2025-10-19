use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::command::Command;
use crate::domain::command::GetCommand;
use crate::domain::domain_error::DomainResult;
use crate::domain::entity::session::Session;
use crate::domain::repository::session::SessionRepository;

pub struct GetSessionCommand {
    id: Uuid,
}
impl GetSessionCommand {
    pub fn new(id_str: &str) -> DomainResult<Self> {
        let id = Uuid::try_parse(id_str).map_err(DomainError::InvalidUuid)?;
        Ok(Self { id })
    }
}
impl Command for GetSessionCommand {}
impl GetCommand<SessionRepository, Session> for GetSessionCommand {
    async fn exec(
        &self,
        repo: &SessionRepository,
    ) -> DomainResult<Session> {
        repo.get(&self.id).await
    }
}
