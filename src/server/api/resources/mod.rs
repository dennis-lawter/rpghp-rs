use super::domain::Record;

pub mod creature;
pub mod session;

pub trait View<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
}
