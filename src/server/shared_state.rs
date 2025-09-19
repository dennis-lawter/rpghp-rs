#[allow(unused_imports)]
use crate::prelude::*;

use handlebars::Handlebars;
use poem::IntoResponse;
use poem::http::StatusCode;
use poem_openapi::payload;

use crate::Config;
use crate::domain::Domain;

#[derive(Clone)]
pub struct SharedState {
    pub domain: Domain,
    pub hb: Handlebars<'static>,
}
impl SharedState {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let mut hb = Handlebars::new();
        Self::register_all_templates(&mut hb)?;

        let domain = Domain::new(cfg).await?;

        Ok(Self { domain, hb })
    }

    #[allow(clippy::needless_pass_by_value)]
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

    fn register_all_templates(hb: &mut Handlebars) -> CrateResult<()> {
        hb.register_template_file("index", "./handlebars/index.hbs")?;
        hb.register_template_file("partials/example", "./handlebars/partials/example.hbs")?;
        Ok(())
    }
}
