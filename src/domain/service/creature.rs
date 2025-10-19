use crate::domain::command::creature::create::CreateCreatureCommand;
use crate::domain::domain_error::DomainResult;
use crate::domain::entity::creature::Creature;
use crate::domain::repository::creature::CreatureRepository;

#[derive(Clone)]
pub struct CreatureService {
    creature_repository: CreatureRepository,
}
impl CreatureService {
    pub async fn create(
        &self,
        _cmd: &CreateCreatureCommand,
    ) -> DomainResult<Creature> {
        todo!()
    }
}
