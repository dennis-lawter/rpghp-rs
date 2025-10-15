use crate::domain::DomainResult;

pub trait Record: Sized {
    async fn find_by_id(
        conn: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> DomainResult<Self>;

    async fn save(
        &self,
        conn: &sqlx::PgPool,
    ) -> DomainResult<()>;

    async fn delete(
        self,
        conn: &sqlx::PgPool,
    ) -> DomainResult<()>;
}
