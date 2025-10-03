#[allow(unused_imports)]
use crate::prelude::*;

use super::domain_actions::DomainResult;

pub mod creature;
pub mod init_group;
pub mod session;

#[allow(dead_code)]
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
