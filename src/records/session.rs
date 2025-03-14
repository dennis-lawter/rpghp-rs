use sqlx::PgPool;
use uuid::Uuid;

use crate::CrateError;
use crate::CrateResult;

use super::Record;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub(crate) struct SessionRecord {
    pub rpghp_session_id: Uuid,
    pub secret: Uuid,
}
impl Record for SessionRecord {
    const TABLE: &'static str = "rpghp_session";

    async fn find_by_id(conn: &PgPool, id: &Uuid) -> crate::CrateResult<Option<Self>> {
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
        .map_err(CrateError::SqlxQueryError)
    }

    async fn save(&self, conn: &PgPool) -> CrateResult<()> {
        sqlx::query!(
            r#"
INSERT INTO
    rpghp_session
    (rpghp_session_id,secret)
    VALUES
    ($1, $2)
ON CONFLICT (rpghp_session_id) DO UPDATE
    SET secret=$2
        "#,
            self.rpghp_session_id,
            self.secret
        )
        .execute(conn)
        .await
        .map_err(CrateError::SqlxQueryError)?;
        Ok(())
    }

    async fn delete(self, conn: &PgPool) -> CrateResult<()> {
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
        .map_err(CrateError::SqlxQueryError)?;
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

    pub async fn find_by_secret(conn: &PgPool, secret: &Uuid) -> crate::CrateResult<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    rpghp_session_id,
    secret
FROM
    rpghp_session
WHERE
    secret = $1
        "#,
            secret
        )
        .fetch_optional(conn)
        .await
        .map_err(CrateError::SqlxQueryError)
    }

    pub async fn find_by_secret_or_id(
        conn: &PgPool,
        secret: &Uuid,
    ) -> crate::CrateResult<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    rpghp_session_id,
    secret
FROM
    rpghp_session
WHERE
    secret = $1
    OR rpghp_session_id = $1
LIMIT 1
        "#,
            secret
        )
        .fetch_optional(conn)
        .await
        .map_err(CrateError::SqlxQueryError)
    }
}
