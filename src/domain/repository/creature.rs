use sqlx::PgPool;

#[allow(dead_code)]
pub struct CreatureRepository {
    db: PgPool,
}
