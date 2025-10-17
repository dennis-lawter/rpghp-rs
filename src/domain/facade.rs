use sqlx::PgPool;

use crate::domain::repository::creature::CreatureRepository;
use crate::domain::repository::session::SessionRepository;
use crate::domain::service::creature::CreatureService;
use crate::domain::service::session::SessionService;

#[derive(Clone)]
pub struct Domain {
    db: PgPool,

    pub session_service: SessionService,
    pub creature_service: CreatureService,
}
impl Domain {
    pub fn new(db: PgPool) -> Self {
        let session_repo = SessionRepository::new(db.clone());
        let creature_repo = CreatureRepository::new(db.clone());

        let session_service = SessionService::new(session_repo.clone());
        let creature_service = CreatureService::new(creature_repo.clone(), session_repo.clone());

        Self {
            db,
            session_service,
            creature_service,
        }
    }
}
