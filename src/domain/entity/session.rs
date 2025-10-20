use uuid::Uuid;

#[allow(dead_code)]
pub struct SessionEntity {
    pub id: Uuid,
    pub secret: Uuid,
}
