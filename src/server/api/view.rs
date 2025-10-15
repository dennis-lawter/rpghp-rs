use crate::domain::records::Record;

pub trait View<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
}
