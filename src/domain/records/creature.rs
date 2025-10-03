#[allow(unused_imports)]
use crate::prelude::*;

use crate::domain::domain_actions::DomainError;
use crate::domain::domain_actions::DomainResult;

use uuid::Uuid;

use super::Record;

pub struct CreatureRecord {
    pub rpghp_creature_id: Uuid,
    pub session_id: Uuid,
    pub rpghp_init_group_id: Uuid,
    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
impl Record for CreatureRecord {
    async fn find_by_id(
        conn: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> DomainResult<Self> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    rpghp_creature_id,
    session_id,
    creature_name,
    max_hp,
    curr_hp,
    hp_hidden,
    icon,
    rpghp_init_group_id
FROM
    rpghp_creature
WHERE
    rpghp_creature_id = $1
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
    rpghp_creature
    (
        rpghp_creature_id,
        session_id,
        creature_name,
        max_hp,
        curr_hp,
        hp_hidden,
        icon,
        rpghp_init_group_id
    )
    VALUES
    (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8
    )
ON CONFLICT (rpghp_creature_id) DO UPDATE
    SET
        session_id = $2,
        creature_name = $3,
        max_hp = $4,
        curr_hp = $5,
        hp_hidden = $6,
        icon = $7,
        rpghp_init_group_id = $8
        "#,
            self.rpghp_creature_id,
            self.session_id,
            self.creature_name,
            self.max_hp,
            self.curr_hp,
            self.hp_hidden,
            self.icon,
            self.rpghp_init_group_id
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
    rpghp_creature
WHERE
    rpghp_creature_id = $1
        "#,
            self.rpghp_creature_id,
        )
        .execute(conn)
        .await
        .map_err(DomainError::SqlxError)?;
        Ok(())
    }
}

impl CreatureRecord {
    pub async fn find_by_session_id(
        conn: &sqlx::PgPool,
        session_id: &uuid::Uuid,
    ) -> DomainResult<Vec<Self>> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    rpghp_creature_id,
    session_id,
    creature_name,
    max_hp,
    curr_hp,
    hp_hidden,
    icon,
    rpghp_init_group_id
FROM
    rpghp_creature
WHERE
    session_id = $1
            "#,
            session_id,
        )
        .fetch_all(conn)
        .await
        .map_err(DomainError::SqlxError)
    }
}
