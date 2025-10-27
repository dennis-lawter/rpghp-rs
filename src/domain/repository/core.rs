use sqlx::PgPool;

use crate::domain::repository::creature::CreatureRepository;
use crate::domain::repository::init_group::InitGroupRepository;
use crate::domain::repository::session::SessionRepository;

#[derive(Clone)]
pub struct RepositoryContext {
    pub session: SessionRepository,
    pub init_group: InitGroupRepository,
    pub creature: CreatureRepository,
}
impl RepositoryContext {
    pub fn new(db: PgPool) -> Self {
        Self {
            session: SessionRepository::new(db.clone()),
            init_group: InitGroupRepository::new(db.clone()),
            creature: CreatureRepository::new(db),
        }
    }
}
