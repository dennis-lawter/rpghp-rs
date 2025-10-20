use uuid::Uuid;

#[allow(dead_code)]
pub struct CreatureEntity {
    pub rpghp_creature_id: Uuid,
    pub session_id: Uuid,
    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
