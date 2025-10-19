use crate::domain::entity::Entity;

pub trait FromEntity<T: Entity>: Sized {
    fn from_entity(entity: &T) -> Self;
}
