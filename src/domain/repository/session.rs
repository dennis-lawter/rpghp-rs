use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::session::SessionEntity;

#[derive(Clone)]
pub struct SessionRepository {
    db: PgPool,
}
impl SessionRepository {
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[allow(dead_code)]
impl SessionRepository {
    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> DomainResult<SessionEntity> {
        sqlx::query_as!(
            SessionEntity,
            r#"
SELECT
    rpghp_session_id as id,
    secret
FROM
    rpghp_session
WHERE
    rpghp_session_id = $1
        "#,
            id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(DomainError::SqlxError)?
        .ok_or(DomainError::NotFound)
    }

    pub async fn create(
        &self,
        entity: &SessionEntity,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
INSERT INTO
    rpghp_session
    (
        rpghp_session_id,
        secret
    )
    VALUES
    (
        $1,
        $2
    )
ON CONFLICT (rpghp_session_id) DO UPDATE
    SET
        secret = $2
        "#,
            entity.id,
            entity.secret
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    pub async fn delete(
        &self,
        entity: &SessionEntity,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
DELETE FROM
    rpghp_session
WHERE
    rpghp_session_id=$1
    AND secret=$2
        "#,
            entity.id,
            entity.secret
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    pub async fn find_by_id_and_secret(
        &self,
        id: &Uuid,
        secret: &Uuid,
    ) -> DomainResult<SessionEntity> {
        let Ok(session) = self.find_by_id(id).await else {
            return Err(DomainError::NotFound);
        };

        if *secret != session.secret {
            return Err(DomainError::Forbidden);
        }

        Ok(session)
    }
}
