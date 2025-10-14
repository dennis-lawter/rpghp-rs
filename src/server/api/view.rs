use crate::domain::records::Record;

pub(super) trait View<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
}
