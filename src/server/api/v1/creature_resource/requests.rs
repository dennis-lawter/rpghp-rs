use crate::domain::command::auth::DomainAuth;
use crate::domain::command::creature::create::CreateCreatureCommand;
use crate::server::api::request::ToCommand;

#[derive(serde::Deserialize, poem_openapi::Object)]
pub struct CreatureCreateRequest {
    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
impl ToCommand<CreateCreatureCommand> for CreatureCreateRequest {
    fn to_command(
        &self,
        domain_auth: &DomainAuth,
    ) -> CreateCreatureCommand {
        CreateCreatureCommand {
            domain_auth: domain_auth.clone(),

            creature_name: self.creature_name.clone(),
            max_hp: self.max_hp,
            curr_hp: self.curr_hp,
            hp_hidden: self.hp_hidden,
            icon: self.icon.clone(),
        }
    }
}
