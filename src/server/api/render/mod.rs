use super::domain::Record;

pub mod session_view;

pub trait View<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
}
