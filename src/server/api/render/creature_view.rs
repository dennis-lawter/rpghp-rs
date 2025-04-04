use poem_openapi::ApiResponse;

#[derive(ApiResponse)]
pub enum CreatureCreateResponse {
    #[oai(status = 201)]
    Created,

    #[oai(status = 404)]
    NotFound,
}
