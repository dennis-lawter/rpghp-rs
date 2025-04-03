use poem_openapi::ApiResponse;

#[derive(ApiResponse)]
pub enum CreatureCreateResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound,
}
