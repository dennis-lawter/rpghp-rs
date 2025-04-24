#[allow(unused_imports)]
use crate::prelude::*;

use crate::domain::domain_actions::DomainError;
use crate::domain::domain_actions::DomainResult;

use sqlx::PgPool;
use uuid::Uuid;

use super::Record;

#[derive(sqlx::FromRow)]
pub struct SessionRecord {
    pub rpghp_session_id: Uuid,
    pub secret: Uuid,
}
impl super::Record for SessionRecord {
    async fn find_by_id(
        conn: &PgPool,
        id: &Uuid,
    ) -> DomainResult<Self> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    rpghp_session_id,
    secret
FROM
    rpghp_session
WHERE
    rpghp_session_id = $1
        "#,
            id
        )
        .fetch_optional(conn)
        .await
        .map_err(DomainError::SqlxError)?
        .ok_or(DomainError::NotFound)
    }

    async fn save(
        &self,
        conn: &PgPool,
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
            self.rpghp_session_id,
            self.secret
        )
        .execute(conn)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    async fn delete(
        self,
        conn: &PgPool,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
DELETE FROM
    rpghp_session
WHERE
    secret=$1
        "#,
            self.secret
        )
        .execute(conn)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }
}

impl SessionRecord {
    pub fn new() -> Self {
        let rpghp_session_id = Uuid::new_v4();
        let secret = Uuid::new_v4();
        Self {
            rpghp_session_id,
            secret,
        }
    }

    pub async fn find_by_id_and_secret(
        pool: &PgPool,
        id: &Uuid,
        secret: &Uuid,
    ) -> DomainResult<Self> {
        let session = match SessionRecord::find_by_id(pool, id).await {
            Ok(session_record) => session_record,
            _ => return Err(DomainError::NotFound),
        };

        if *secret != session.secret {
            return Err(DomainError::Forbidden);
        }

        Ok(session)
    }
}
