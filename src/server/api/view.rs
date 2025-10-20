use crate::domain::entity::Entity;

pub trait View<T: Entity>: Sized {
    fn from_entity(record: &T) -> Self;
}
