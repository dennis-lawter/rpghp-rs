use sqlx::PgPool;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::init_group::InitGroupEntity;

#[derive(Clone)]
#[allow(dead_code)]
pub struct InitGroupRepository {
    db: PgPool,
}
impl InitGroupRepository {
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        entity: &InitGroupEntity,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
INSERT INTO
    rpghp_init_group
    (
        rpghp_init_group_id,
        session_id,
        rank
    )
    VALUES
    (
        $1,
        $2,
        $3
    )
ON CONFLICT (rpghp_init_group_id) DO UPDATE
    SET
        session_id = $2,
        rank = $3
            "#,
            entity.rpghp_init_group_id,
            entity.session_id,
            entity.rank
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    pub(crate) async fn find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> DomainResult<InitGroupEntity> {
        sqlx::query_as!(
            InitGroupEntity,
            r#"
SELECT
    *
FROM
    rpghp_init_group
WHERE
    rpghp_init_group_id = $1
            "#,
            id,
        )
        .fetch_optional(&self.db)
        .await
        .map_err(DomainError::SqlxError)?
        .ok_or(DomainError::NotFound)
    }
}
