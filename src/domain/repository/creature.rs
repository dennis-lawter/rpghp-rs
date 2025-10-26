use sqlx::PgPool;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::creature::CreatureEntity;

#[derive(Clone)]
pub struct CreatureRepository {
    db: PgPool,
}
impl CreatureRepository {
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn find_by_id(
        &self,
        id: &uuid::Uuid,
    ) -> DomainResult<CreatureEntity> {
        sqlx::query_as!(
            CreatureEntity,
            r#"
SELECT
    rpghp_creature_id,
    creature_name,
    max_hp,
    curr_hp,
    hp_hidden,
    icon,
    init_group_id
FROM
    rpghp_creature
WHERE
    rpghp_creature_id = $1
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
        entity: &CreatureEntity,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
INSERT INTO
    rpghp_creature
    (
        rpghp_creature_id,
        creature_name,
        max_hp,
        curr_hp,
        hp_hidden,
        icon,
        init_group_id
    )
    VALUES
    (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7
    )
ON CONFLICT (rpghp_creature_id) DO UPDATE
    SET
        creature_name = $2,
        max_hp = $3,
        curr_hp = $4,
        hp_hidden = $5,
        icon = $6,
        init_group_id = $7
        "#,
            entity.rpghp_creature_id,
            entity.creature_name,
            entity.max_hp,
            entity.curr_hp,
            entity.hp_hidden,
            entity.icon,
            entity.init_group_id,
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    // TODO: impl delete endpoint, but consider soft deletes?
    #[allow(dead_code)]
    pub async fn delete(
        self,
        entity: CreatureEntity,
    ) -> DomainResult<()> {
        sqlx::query!(
            r#"
DELETE FROM
    rpghp_creature
WHERE
    rpghp_creature_id = $1
        "#,
            entity.rpghp_creature_id,
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    pub async fn find_by_session_id(
        &self,
        session_id: &uuid::Uuid,
    ) -> DomainResult<Vec<CreatureEntity>> {
        sqlx::query_as!(
            CreatureEntity,
            r#"
SELECT
    c.rpghp_creature_id,
    c.creature_name,
    c.max_hp,
    c.curr_hp,
    c.hp_hidden,
    c.icon,
    c.init_group_id
FROM
    rpghp_creature c
LEFT JOIN
    rpghp_init_group ig
    ON c.init_group_id = ig.rpghp_init_group_id
WHERE
    ig.session_id = $1
            "#,
            session_id,
        )
        .fetch_all(&self.db)
        .await
        .map_err(DomainError::SqlxError)
    }
}
