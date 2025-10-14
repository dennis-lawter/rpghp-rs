use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;

use super::views::SessionView;
use super::views::SessionWithSecretView;
use crate::domain::DomainError;

#[derive(ApiResponse)]
pub(super) enum SessionCreateResponse {
    #[oai(status = 201)]
    Ok(Json<SessionWithSecretView>),
    #[oai(status = 404)]
    NotFound,
}
impl SessionCreateResponse {
    pub(super) const fn from_domain_error(_err: &DomainError) -> Self {
        Self::NotFound
    }
}

#[derive(ApiResponse)]
pub(super) enum SessionGetResponse {
    #[oai(status = 200)]
    Ok(Json<SessionView>),
    #[oai(status = 404)]
    NotFound,
}
impl SessionGetResponse {
    pub(super) const fn from_domain_error(_err: &DomainError) -> Self {
        Self::NotFound
    }
}

#[derive(ApiResponse)]
pub(super) enum SessionDeleteResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 403)]
    Forbidden,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
impl SessionDeleteResponse {
    pub(super) const fn from_domain_error(err: &DomainError) -> Self {
        match err {
            DomainError::NotFound => Self::NotFound,
            DomainError::Forbidden => Self::Forbidden,
            DomainError::SqlxError(_) => Self::InternalError,
            DomainError::InvalidUuid(_) => Self::BadRequest,
        }
    }
}
