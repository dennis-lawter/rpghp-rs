use crate::prelude::*;

use handlebars::Handlebars;
use poem::IntoResponse;
use poem::http::StatusCode;
use poem_openapi::payload;

#[derive(Clone)]
pub struct FrontendSharedState {
    pub hb: Handlebars<'static>,
}
impl FrontendSharedState {
    pub fn new() -> CrateResult<Self> {
        let mut hb = Handlebars::new();
        Self::register_all_templates(&mut hb)?;
        Ok(Self { hb })
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

    fn register_all_templates(hb: &mut Handlebars) -> CrateResult<()> {
        hb.register_template_file("index", "./handlebars/index.hbs")?;
        hb.register_template_file("partials/example", "./handlebars/partials/example.hbs")?;
        Ok(())
    }
}
