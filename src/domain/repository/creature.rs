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
    rpghp_creature_id as id,
    session_id,
    creature_name,
    max_hp,
    curr_hp,
    hp_hidden,
    icon
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
        session_id,
        creature_name,
        max_hp,
        curr_hp,
        hp_hidden,
        icon
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
        session_id = $2,
        creature_name = $3,
        max_hp = $4,
        curr_hp = $5,
        hp_hidden = $6,
        icon = $7
        "#,
            entity.id,
            entity.session_id,
            entity.creature_name,
            entity.max_hp,
            entity.curr_hp,
            entity.hp_hidden,
            entity.icon,
        )
        .execute(&self.db)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }

    //     pub async fn delete(
    //         self,
    //         entity: CreatureEntity,
    //     ) -> DomainResult<()> {
    //         sqlx::query!(
    //             r#"
    // DELETE FROM
    //     rpghp_creature
    // WHERE
    //     rpghp_creature_id = $1
    //         "#,
    //             entity.id,
    //         )
    //         .execute(&self.db)
    //         .await
    //         .map_err(DomainError::SqlxError)?;
    //         Ok(())
    //     }

    pub async fn find_by_session_id(
        &self,
        session_id: &uuid::Uuid,
    ) -> DomainResult<Vec<CreatureEntity>> {
        sqlx::query_as!(
            CreatureEntity,
            r#"
SELECT
    rpghp_creature_id as id,
    session_id,
    creature_name,
    max_hp,
    curr_hp,
    hp_hidden,
    icon
FROM
    rpghp_creature
WHERE
    session_id = $1
            "#,
            session_id,
        )
        .fetch_all(&self.db)
        .await
        .map_err(DomainError::SqlxError)
    }
}
