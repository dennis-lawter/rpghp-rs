use sqlx::types::Uuid;

use crate::CrateError;

use super::Record;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub(crate) struct SessionRecord {
    #[sqlx(rename = "id")]
    pub rpghp_session_id: Uuid,
    pub secret: Uuid,
}
impl Record for SessionRecord {
    const TABLE: &'static str = "rpghp_session";

    async fn find_by_id(
        conn: &mut sqlx::PgConnection,
        id: &Uuid,
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
    rpghp_session_id = $1
        "#,
            id
        )
        .fetch_optional(conn)
        .await
        .map_err(CrateError::SqlxQueryError)
    }
}
