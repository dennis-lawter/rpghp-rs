use crate::domain::records::init_group::InitGroupRecord;
#[allow(unused_imports)]
use crate::prelude::*;

use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;

use super::records::Record;
use super::records::creature::CreatureRecord;
use super::records::session::SessionRecord;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("Not found")]
    NotFound,
    // #[error("Invalid auth provided")]
    // Unauthorized,
    #[error("Provided auth does not grant permission for requested record")]
    Forbidden,
    #[error("SQL error: {0}")]
    SqlxError(sqlx::Error),
    #[error("Invalid UUID")]
    InvalidUuid(uuid::Error),
}
pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Clone)]
pub struct Domain {
    db: PgPool,
}

// Creation
impl Domain {
    pub async fn new(cfg: &Config) -> CrateResult<Self> {
        let db = Self::get_db_pool(cfg).await?;
        Self::migrate_db(&db).await?;

        Ok(Self { db })
    }

    async fn get_db_pool(cfg: &Config) -> CrateResult<PgPool> {
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
}

// Utility
// impl Domain {}

// Session
impl Domain {
    pub async fn create_session(&self) -> DomainResult<SessionRecord> {
        let session_record = SessionRecord::new();
        session_record.save(&self.db).await?;
        Ok(session_record)
    }

    pub async fn get_session(
        &self,
        id: &str,
    ) -> DomainResult<SessionRecord> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        SessionRecord::find_by_id(&self.db, &id).await
    }

    pub async fn delete_session(
        &self,
        id: &str,
        secret: &str,
    ) -> DomainResult<()> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let session = SessionRecord::find_by_id_and_secret(&self.db, &id, &secret).await?;
        session.delete(&self.db).await
    }
}

// Creature

impl Domain {
    pub async fn create_creature(
        &self,
        id: &str,
        secret: &str,
        creature_name: &str,
        max_hp: i32,
        curr_hp: i32,
        hp_hidden: bool,
        icon: Option<String>,
        init_group_id: &str,
    ) -> DomainResult<CreatureRecord> {
        // TODO: Add option around init group; create new when empty
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let init_group_id = Uuid::parse_str(init_group_id).map_err(DomainError::InvalidUuid)?;
        let session = SessionRecord::find_by_id_and_secret(&self.db, &id, &secret).await?;
        let init_group = InitGroupRecord::find_by_id(&self.db, &init_group_id).await?;
        let creature = CreatureRecord {
            rpghp_creature_id: Uuid::new_v4(),
            session_id: session.rpghp_session_id,
            creature_name: String::from(creature_name),
            max_hp,
            curr_hp,
            hp_hidden,
            icon,
            rpghp_init_group_id: init_group.rpghp_init_group_id,
        };
        creature.save(&self.db).await?;

        Ok(creature)
    }

    pub async fn get_all_creatures_for_session(
        &self,
        id: &str,
        opt_secret: Option<&String>,
    ) -> DomainResult<Vec<CreatureRecord>> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        let session = match opt_secret {
            None => SessionRecord::find_by_id(&self.db, &id).await?,
            Some(token) => {
                let token = Uuid::parse_str(token).map_err(DomainError::InvalidUuid)?;
                SessionRecord::find_by_id_and_secret(&self.db, &id, &token).await?
            }
        };

        let Ok(creatures) =
            CreatureRecord::find_by_session_id(&self.db, &session.rpghp_session_id).await
        else {
            return Err(DomainError::NotFound);
        };

        Ok(creatures)
    }

    pub async fn get_creature(
        &self,
        session_id: &str,
        creature_id: &str,
        opt_secret: Option<&String>,
    ) -> DomainResult<CreatureRecord> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let creature_id = Uuid::parse_str(creature_id).map_err(DomainError::InvalidUuid)?;
        let session = match opt_secret {
            None => SessionRecord::find_by_id(&self.db, &session_id).await?,
            Some(token) => {
                let token = Uuid::parse_str(token).map_err(DomainError::InvalidUuid)?;
                SessionRecord::find_by_id_and_secret(&self.db, &session_id, &token).await?
            }
        };
        let creature = CreatureRecord::find_by_id(&self.db, &creature_id).await?;
        if creature.session_id != session.rpghp_session_id {
            return Err(DomainError::Forbidden);
        }
        Ok(creature)
    }
}

// Initiative groups

impl Domain {
    // TODO: Make rank optional, default to a tail insertion
    pub async fn create_init_group(
        &self,
        session_id: &str,
        secret: &str,
        rank: i64,
    ) -> DomainResult<()> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let _session = SessionRecord::find_by_id_and_secret(&self.db, &session_id, &secret).await?;
        let id = Uuid::new_v4();
        let init_group = InitGroupRecord {
            rpghp_init_group_id: id,
            session_id,
            rank,
        };
        init_group.save(&self.db).await?;
        Ok(())
    }
}
