use sqlx::PgPool;

#[derive(Clone)]
pub struct CreatureRepository {
    db: PgPool,
}
impl CreatureRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
