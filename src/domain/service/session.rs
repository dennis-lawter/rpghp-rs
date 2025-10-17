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

    pub async fn create(&self) -> DomainResult<Session> {
        let session = Session::new();
        self.session_repo.create(&session).await?;
        Ok(session)
    }
}
