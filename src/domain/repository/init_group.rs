use sqlx::PgPool;

#[derive(Clone)]
#[allow(dead_code)]
pub struct InitGroupRepository {
    db: PgPool,
}
impl InitGroupRepository {
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }
}
