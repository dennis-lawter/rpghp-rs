mod endpoint;
pub mod requests;
mod responses;
mod views;

pub use endpoint::*;

// use crate::domain::DomainError;
// use crate::domain::command::auth::DomainAuth;
// use crate::server::api::v1::error_handling::FromDomainError;
//
// async fn handle_domain_call<Resp, Fut>(
//     make_auth: Result<DomainAuth, DomainError>,
//     f: Fut,
// ) -> Resp
// where
//     Resp: FromDomainError,
//     Fut: Future<Output = Result<(), DomainError>>,
// {
//     let domain_auth = match make_auth {
//         Ok(a) => a,
//         Err(e) => return Resp::from_domain_error(&e),
//     };
//     match f.await {
//         Ok(_) => Resp::success(),
//         Err(e) => Resp::from_domain_error(&e),
//     }
// }
