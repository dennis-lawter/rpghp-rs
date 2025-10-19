#[derive(serde::Deserialize, poem_openapi::Object)]
pub struct CreatureCreateRequest {
    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
