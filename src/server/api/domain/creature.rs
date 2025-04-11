use crate::prelude::*;

use uuid::Uuid;

use super::Record;

pub struct CreatureRecord {
    pub rpghp_creature_id: Uuid,
    pub session_id: Uuid,
    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
}
impl Record for CreatureRecord {
    async fn find_by_id(
        conn: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> crate::prelude::CrateResult<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
SELECT
    rpghp_creature_id,
    session_id,
    creature_name,
    max_hp,
    curr_hp,
    hp_hidden
FROM
    rpghp_creature
WHERE
    rpghp_creature_id = $1
        "#,
            id
        )
        .fetch_optional(conn)
        .await
        .map_err(CrateError::SqlxError)
    }

    async fn save(
        &self,
        conn: &sqlx::PgPool,
    ) -> crate::prelude::CrateResult<()> {
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
        hp_hidden
    )
    VALUES
    (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6
    )
ON CONFLICT (rpghp_creature_id) DO UPDATE
    SET
        session_id = $2,
        creature_name = $3,
        max_hp = $4,
        curr_hp = $5,
        hp_hidden = $6
        "#,
            self.rpghp_creature_id,
            self.session_id,
            self.creature_name,
            self.max_hp,
            self.curr_hp,
            self.hp_hidden,
        )
        .execute(conn)
        .await
        .map_err(CrateError::SqlxError)?;
        Ok(())
    }

    async fn delete(
        self,
        conn: &sqlx::PgPool,
    ) -> crate::prelude::CrateResult<()> {
        sqlx::query!(
            r#"
DELETE FROM
    rpghp_creature
WHERE
    rpghp_creature_id=$1
        "#,
            self.rpghp_creature_id,
        )
        .execute(conn)
        .await
        .map_err(CrateError::SqlxError)?;
        Ok(())
    }
}
