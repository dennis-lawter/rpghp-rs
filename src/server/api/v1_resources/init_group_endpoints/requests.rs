#[derive(serde::Deserialize, poem_openapi::Object)]
pub struct CreateInitGroupRequest {
    pub rank: Option<i64>,
}
