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
}
