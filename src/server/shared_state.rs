use crate::prelude::*;

use handlebars::Handlebars;
use poem::IntoResponse;
use poem::http::StatusCode;
use poem_openapi::payload;
use sqlx::PgPool;

use crate::Config;

#[derive(Clone)]
pub struct SharedState {
    pub pool: PgPool,
    pub hb: Handlebars<'static>,
}
impl SharedState {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let pool = Self::get_pool(cfg).await?;
        Self::migrate_db(&pool).await?;

        let mut hb = Handlebars::new();
        Self::register_all_templates(&mut hb)?;

        Ok(Self { pool, hb })
    }

    async fn get_pool(cfg: &Config) -> CrateResult<PgPool> {
        sqlx::Pool::<sqlx::Postgres>::connect(&cfg.db_url)
            .await
            .map_err(CrateError::SqlxError)
    }

    async fn migrate_db(pool: &PgPool) -> CrateResult<()> {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .map_err(CrateError::SqlxMigrationError)?;
        Ok(())
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
