use poem_openapi::Object;

use crate::domain::records::creature::CreatureRecord;
use crate::server::api::view::View;

#[derive(Object, serde::Serialize)]
pub(super) struct CreatureView {
    creature_id: String,
    creature_name: String,
    max_hp: Option<i32>,
    curr_hp: Option<i32>,
    approx_hp: f32,
    hp_hidden: bool,
}
impl View<CreatureRecord> for CreatureView {
    fn from_record(record: &CreatureRecord) -> Self {
        let id = format!("{}", record.rpghp_creature_id);
        #[allow(clippy::cast_precision_loss)]
        let approx_hp = record.curr_hp as f32 / record.max_hp as f32;
        Self {
            creature_id: id,
            creature_name: record.creature_name.clone(),
            max_hp: Some(record.max_hp),
            curr_hp: Some(record.curr_hp),
            approx_hp,
            hp_hidden: record.hp_hidden,
        }
    }
}
impl CreatureView {
    pub(super) fn simplified_if_hp_hidden(self) -> Self {
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
