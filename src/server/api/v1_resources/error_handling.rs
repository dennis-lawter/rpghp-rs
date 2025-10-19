use crate::domain::DomainError;

pub trait FromDomainError {
    fn from_domain_error(err: &DomainError) -> Self;
}
