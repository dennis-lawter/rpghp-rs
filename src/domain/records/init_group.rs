#[allow(unused_imports)]
use crate::prelude::*;

use crate::domain::domain_actions::DomainError;
use crate::domain::domain_actions::DomainResult;

use uuid::Uuid;

use super::Record;

pub struct InitGroupRecord {
    pub rpghp_init_group_id: Uuid,
    pub rank: i64,
}
impl Record for InitGroupRecord {
    async fn find_by_id(
        conn: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> DomainResult<Self> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    *
FROM
    rpghp_init_group
WHERE
    rpghp_init_group_id = $1
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
        conn: &sqlx::PgPool,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
INSERT INTO
    rpghp_init_group
    (
        rpghp_init_group_id,
        rank
    )
    VALUES
    (
        $1,
        $2
    )
ON CONFLICT (rpghp_init_group_id) DO UPDATE
    SET
        rank = $2
        "#,
            self.rpghp_init_group_id,
            self.rank
        )
        .execute(conn)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    async fn delete(
        self,
        conn: &sqlx::PgPool,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
DELETE FROM
    rpghp_init_group
WHERE
    rpghp_init_group_id = $1
        "#,
            self.rpghp_init_group_id,
        )
        .execute(conn)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }
}
