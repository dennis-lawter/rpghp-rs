use crate::domain::domain_error::DomainResult;
use crate::domain::entity::Entity;
use crate::domain::repository::Repository;

pub mod auth;
pub mod creature;
pub mod session;

pub trait Command {}

pub trait GetCommand<R: Repository, T: Entity> {
    async fn exec(
        &self,
        repo: &R,
    ) -> DomainResult<T>;
}
pub trait ListCommand<R: Repository, T: Entity> {
    async fn exec(
        &self,
        repo: &R,
    ) -> DomainResult<Vec<T>>;
}
pub trait CreateCommand<R: Repository, T: Entity> {
    async fn exec(
        &self,
        repo: &R,
    ) -> DomainResult<T>;
}
pub trait UpdateCommand<R: Repository> {
    async fn exec(
        &self,
        repo: &R,
    ) -> DomainResult<()>;
}
pub trait DeleteCommand<R: Repository> {
    async fn exec(
        &self,
        repo: &R,
    ) -> DomainResult<()>;
}
