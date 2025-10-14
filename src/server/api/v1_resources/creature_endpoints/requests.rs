#[derive(serde::Deserialize, poem_openapi::Object)]
pub(super) struct CreatureCreateRequest {
    pub(super) creature_name: String,
    pub(super) max_hp: i32,
    pub(super) curr_hp: i32,
    pub(super) hp_hidden: bool,
    pub(super) icon: Option<String>,
}
