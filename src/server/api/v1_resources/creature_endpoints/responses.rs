use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;

use super::views::CreatureView;
use crate::domain::DomainError;
use crate::server::api::v1_resources::error_handling::FromDomainError;

#[derive(ApiResponse, Clone, Copy, Debug)]
pub enum CreateCreatureResponse {
    #[oai(status = 201)]
    Created,
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 403)]
    Forbidden,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
impl FromDomainError for CreateCreatureResponse {
    fn from_domain_error(err: &DomainError) -> Self {
        match err {
            DomainError::NotFound => Self::NotFound,
            DomainError::Forbidden => Self::Forbidden,
            DomainError::SqlxError(e) => {
                log::error!("{e}");
                Self::InternalError
            }
            DomainError::InvalidUuid(_) => Self::BadRequest,
        }
    }
}

#[derive(ApiResponse, Clone, Debug)]
pub enum ListCreatureResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<CreatureView>>),
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 403)]
    Forbidden,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
impl FromDomainError for ListCreatureResponse {
    fn from_domain_error(err: &DomainError) -> Self {
        match err {
            DomainError::NotFound => Self::NotFound,
            DomainError::Forbidden => Self::Forbidden,
            DomainError::SqlxError(e) => {
                log::error!("{e}");
                Self::InternalError
            }
            DomainError::InvalidUuid(_) => Self::BadRequest,
        }
    }
}

#[derive(ApiResponse, Clone, Debug)]
pub enum GetCreatureResponse {
    #[oai(status = 200)]
    Ok(Json<CreatureView>),
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 403)]
    Forbidden,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
impl FromDomainError for GetCreatureResponse {
    fn from_domain_error(err: &DomainError) -> Self {
        match err {
            DomainError::NotFound => Self::NotFound,
            DomainError::Forbidden => Self::Forbidden,
            DomainError::SqlxError(e) => {
                log::error!("{e}");
                Self::InternalError
            }
            DomainError::InvalidUuid(_) => Self::BadRequest,
        }
    }
}
