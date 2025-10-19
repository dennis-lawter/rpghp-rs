use crate::domain::command::Command;
use crate::domain::command::CreateCommand;
use crate::domain::domain_error::DomainResult;
use crate::domain::entity::session::Session;
use crate::domain::repository::session::SessionRepository;

pub struct CreateSessionCommand {}
impl Command for CreateSessionCommand {}
impl CreateCommand<SessionRepository, Session> for CreateSessionCommand {
    async fn exec(
        &self,
        repo: &SessionRepository,
    ) -> DomainResult<Session> {
        let session = Session::new();
        repo.create(&session).await?;
        Ok(session)
    }
}
