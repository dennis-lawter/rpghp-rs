use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;

use crate::domain::DomainError;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::v1_resources::init_group_endpoints::views::InitGroupView;

#[derive(ApiResponse, Clone, Debug)]
pub enum CreateInitGroupResponse {
    #[oai(status = 201)]
    Created(Json<InitGroupView>),
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 403)]
    Forbidden,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
impl FromDomainError for CreateInitGroupResponse {
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
