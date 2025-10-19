use crate::domain::command::CreateCommand;
use crate::domain::command::DeleteCommand;
use crate::domain::command::GetCommand;
use crate::domain::command::session::create::CreateSessionCommand;
use crate::domain::command::session::delete::DeleteSessionCommand;
use crate::domain::command::session::get::GetSessionCommand;
use crate::domain::domain_error::DomainResult;
use crate::domain::entity::session::Session;
use crate::domain::repository::session::SessionRepository;

#[derive(Clone)]
pub struct SessionService {
    session_repo: SessionRepository,
}
impl SessionService {
    pub fn new(session_repo: SessionRepository) -> Self {
        Self { session_repo }
    }

    pub async fn create(
        &self,
        cmd: &CreateSessionCommand,
    ) -> DomainResult<Session> {
        cmd.exec(&self.session_repo).await
    }

    pub async fn get(
        &self,
        cmd: &GetSessionCommand,
    ) -> DomainResult<Session> {
        cmd.exec(&self.session_repo).await
    }

    pub async fn delete(
        &self,
        cmd: &DeleteSessionCommand,
    ) -> DomainResult<()> {
        cmd.exec(&self.session_repo).await
    }
}
