use crate::prelude::*;
use crate::server::api::api_shared_state::ApiSharedState;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use sqlx::PgPool;
use uuid::Uuid;

use super::Record;
use super::View;

pub struct ApiSessionRoutesV1;
#[OpenApi]
impl ApiSessionRoutesV1 {
    #[oai(path = "/session", method = "post")]
    async fn create_session(
        &self,
        state: Data<&ApiSharedState>,
    ) -> SessionCreateResponse {
        let session_record = SessionRecord::new();
        let res = session_record.save(&state.pool).await;
        let view = SessionWithSecretView::from_record(&session_record);

        match res {
            Ok(_) => SessionCreateResponse::Ok(Json(view)),
            Err(_) => SessionCreateResponse::NotFound,
        }
    }

    #[oai(path = "/session/:session_id", method = "get")]
    async fn get_session(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
    ) -> SessionGetResponse {
        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return SessionGetResponse::NotFound,
        };

        match SessionRecord::find_by_secret_or_id(&state.pool, &uuid).await {
            Ok(Some(session_record)) => {
                let view = SessionView::from_record(&session_record);
                SessionGetResponse::Ok(Json(view))
            }
            _ => SessionGetResponse::NotFound,
        }
    }

    #[oai(path = "/session/:session_id", method = "delete")]
    async fn delete_session(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
    ) -> SessionDeleteResponse {
        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return SessionDeleteResponse::NotFound,
        };

        match SessionRecord::find_by_secret(&state.pool, &uuid).await {
            Ok(Some(session_record)) => match session_record.delete(&state.pool).await {
                Ok(()) => SessionDeleteResponse::Created,
                Err(_) => SessionDeleteResponse::NotFound,
            },
            _ => SessionDeleteResponse::NotFound,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct SessionRecord {
    pub rpghp_session_id: Uuid,
    pub secret: Uuid,
}
impl super::Record for SessionRecord {
    const TABLE: &'static str = "rpghp_session";

    async fn find_by_id(
        conn: &PgPool,
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
        .map_err(CrateError::SqlxError)
    }

    async fn save(
        &self,
        conn: &PgPool,
    ) -> CrateResult<()> {
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
        .map_err(CrateError::SqlxError)?;
        Ok(())
    }

    async fn delete(
        self,
        conn: &PgPool,
    ) -> CrateResult<()> {
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
        .map_err(CrateError::SqlxError)?;
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

    pub async fn find_by_secret(
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
        "#,
            secret
        )
        .fetch_optional(conn)
        .await
        .map_err(CrateError::SqlxError)
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
        .map_err(CrateError::SqlxError)
    }
}

#[derive(ApiResponse)]
enum SessionCreateResponse {
    #[oai(status = 200)]
    Ok(Json<SessionWithSecretView>),

    #[oai(status = 404)]
    NotFound,
}
#[derive(Object, serde::Serialize)]
struct SessionWithSecretView {
    pub rpghp_session_id: String,
    pub secret: String,
}
impl super::View<SessionRecord> for SessionWithSecretView {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        let secret = format!("{}", record.secret);
        Self {
            rpghp_session_id,
            secret,
        }
    }
}

#[derive(ApiResponse)]
enum SessionGetResponse {
    #[oai(status = 200)]
    Ok(Json<SessionView>),

    #[oai(status = 404)]
    NotFound,
}
#[derive(Object, serde::Serialize)]
struct SessionView {
    pub rpghp_session_id: String,
}
impl super::View<SessionRecord> for SessionView {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        Self { rpghp_session_id }
    }
}

#[derive(ApiResponse)]
enum SessionDeleteResponse {
    #[oai(status = 201)]
    Created,

    #[oai(status = 404)]
    NotFound,
}
