mod template_registry;

use handlebars::Handlebars;
use poem::IntoResponse;
use poem::http::StatusCode;
use poem_openapi::payload;

use crate::Config;
use crate::domain::Domain;
use crate::prelude::*;

#[derive(Clone)]
pub struct SharedState {
    pub domain: Domain,
    pub hb: Handlebars<'static>,
}
impl SharedState {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let domain = Domain::new(cfg).await?;

        let mut hb = Handlebars::new();
        template_registry::register_hbs_files_from_dir(&mut hb, "./handlebars")?;

        Ok(Self { domain, hb })
    }

    pub fn render(
        &self,
        id: &str,
        json: serde_json::Value,
    ) -> poem::Result<impl IntoResponse> {
        let html = self.hb.render(id, &json).map_err(|e| {
            poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
        })?;

        Ok(payload::Html(html))
    }
}
