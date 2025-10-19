use poem_openapi::Object;

use crate::domain::entity::creature::Creature;
use crate::server::api::view::FromEntity;

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct CreatureView {
    creature_id: String,
    creature_name: String,
    max_hp: Option<i32>,
    curr_hp: Option<i32>,
    approx_hp: f32,
    hp_hidden: bool,
}
impl FromEntity<Creature> for CreatureView {
    fn from_entity(entity: &Creature) -> Self {
        let id = format!("{}", entity.id);
        #[allow(clippy::cast_precision_loss)]
        let approx_hp = entity.curr_hp as f32 / entity.max_hp as f32;
        Self {
            creature_id: id,
            creature_name: entity.name.clone(),
            max_hp: Some(entity.max_hp),
            curr_hp: Some(entity.curr_hp),
            approx_hp,
            hp_hidden: entity.hp_hidden,
        }
    }
}
impl CreatureView {
    pub fn restricted_view(self) -> Self {
        if self.hp_hidden {
            Self {
                creature_id: self.creature_id,
                creature_name: self.creature_name,
                max_hp: None,
                curr_hp: None,
                approx_hp: self.approx_hp,
                hp_hidden: self.hp_hidden,
            }
        } else {
            self
        }
    }
}
