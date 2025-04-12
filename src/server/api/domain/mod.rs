pub mod creature;
pub mod session;

use crate::prelude::*;

pub enum RecordQueryError {
    NotFound,
    Unauthorized,
    Forbidden,
}

#[allow(dead_code)]
pub trait Record: Sized {
    async fn find_by_id(
        conn: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> CrateResult<Option<Self>>;

    async fn save(
        &self,
        conn: &sqlx::PgPool,
    ) -> CrateResult<()>;

    async fn delete(
        self,
        conn: &sqlx::PgPool,
    ) -> CrateResult<()>;
}
