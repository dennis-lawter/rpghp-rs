use sqlx::PgPool;

use crate::domain::repository::creature::CreatureRepository;
use crate::domain::repository::init_group::InitGroupRepository;
use crate::domain::repository::session::SessionRepository;

#[derive(Clone)]
#[allow(dead_code)]
pub struct InitGroupService {
    init_group_repo: InitGroupRepository,
    creature_repo: CreatureRepository,
    session_repo: SessionRepository,
}
impl InitGroupService {
    pub fn new(db: PgPool) -> Self {
        let init_group_repo = InitGroupRepository::new(db.clone());
        let creature_repo = CreatureRepository::new(db.clone());
        let session_repo = SessionRepository::new(db);
        Self {
            init_group_repo,
            creature_repo,
            session_repo,
        }
    }
}
