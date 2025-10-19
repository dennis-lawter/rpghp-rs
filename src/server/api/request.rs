use crate::domain::command::Command;
use crate::domain::command::auth::DomainAuth;

pub trait ToCommand<T: Command> {
    fn to_command(
        &self,
        domain_auth: &DomainAuth,
    ) -> T;
}
