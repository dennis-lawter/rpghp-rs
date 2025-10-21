use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;

use super::views::SessionWithSecretView;
use super::views::SessionWithoutSecretView;
use crate::domain::DomainError;
use crate::server::api::v1_resources::error_handling::FromDomainError;

#[derive(ApiResponse, Clone, Debug)]
pub enum CreateSessionResponse {
    #[oai(status = 201)]
    Ok(Json<SessionWithSecretView>),
    #[oai(status = 404)]
    NotFound,
}
impl FromDomainError for CreateSessionResponse {
    fn from_domain_error(_err: &DomainError) -> Self {
        Self::NotFound
    }
}

#[derive(ApiResponse, Clone, Debug)]
pub enum GetSessionResponse {
    #[oai(status = 200)]
    Ok(Json<SessionWithoutSecretView>),
    #[oai(status = 404)]
    NotFound,
}
impl FromDomainError for GetSessionResponse {
    fn from_domain_error(_err: &DomainError) -> Self {
        Self::NotFound
    }
}

#[derive(ApiResponse, Clone, Copy, Debug)]
pub enum DeleteSessionResponse {
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
impl FromDomainError for DeleteSessionResponse {
    fn from_domain_error(err: &DomainError) -> Self {
        match err {
            DomainError::NotFound => Self::NotFound,
            DomainError::Forbidden => Self::Forbidden,
            DomainError::SqlxError(_) => Self::InternalError,
            DomainError::InvalidUuid(_) => Self::BadRequest,
        }
    }
}
