use crate::domain::entity::Entity;

pub trait View {}

pub trait FromEntity<T: Entity>: View {
    fn from_entity(entity: &T) -> Self;
}
