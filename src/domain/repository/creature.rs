use sqlx::PgPool;

use crate::domain::domain_error::DomainResult;
use crate::domain::entity::creature::Creature;
use crate::domain::repository::Repository;

#[derive(Clone)]
pub struct CreatureRepository {
    db: PgPool,
}
impl Repository for CreatureRepository {}
impl CreatureRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        _creature: &Creature,
    ) -> DomainResult<()> {
        todo!()
    }
}
