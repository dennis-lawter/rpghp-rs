use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::domain_error::DomainResult;
use crate::domain::entity::session::Session;
use crate::domain::repository::Repository;

#[derive(Clone)]
pub struct SessionRepository {
    db: PgPool,
}
impl Repository for SessionRepository {}
impl SessionRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        session: &Session,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
insert into
    rpghp_session
(rpghp_session_id, secret)
values
($1, $2)
            "#,
            session.id,
            session.secret
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    pub async fn get(
        &self,
        id: &Uuid,
    ) -> DomainResult<Session> {
        sqlx::query_as!(
            Session,
            r#"
select
    rpghp_session_id as id,
    secret
from
    rpghp_session
where
    rpghp_session_id = $1
            "#,
            &id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(DomainError::SqlxError)?
        .ok_or(DomainError::NotFound)
    }

    pub(crate) async fn delete(
        &self,
        id: &Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
delete from
    rpghp_session
where
    rpghp_session_id = $1
            "#,
            id
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }
}
