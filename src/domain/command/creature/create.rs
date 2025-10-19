use crate::domain::command::Command;
use crate::domain::command::CreateCommand;
use crate::domain::command::auth::DomainAuth;
use crate::domain::domain_error::DomainResult;
use crate::domain::entity::creature::Creature;
use crate::domain::repository::creature::CreatureRepository;

pub struct CreateCreatureCommand {
    pub domain_auth: DomainAuth,

    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
impl Command for CreateCreatureCommand {}
impl CreateCommand<CreatureRepository, Creature> for CreateCreatureCommand {
    async fn exec(
        &self,
        _repo: &CreatureRepository,
    ) -> DomainResult<Creature> {
        todo!()
    }
}
