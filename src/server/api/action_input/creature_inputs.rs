#[derive(serde::Deserialize, poem_openapi::Object)]
pub struct CreateCreatureRequest {
    creature_name: String,
    max_hp: i32,
    curr_hp: i32,
    hp_hidden: bool,
}
