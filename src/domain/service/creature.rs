use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::creature::CreatureEntity;
use crate::domain::repository::creature::CreatureRepository;
use crate::domain::repository::init_group::InitGroupRepository;
use crate::domain::repository::session::SessionRepository;

#[derive(Clone)]
pub struct CreatureService {
    creature_repo: CreatureRepository,
    session_repo: SessionRepository,
    init_group_repo: InitGroupRepository,
}
impl CreatureService {
    pub fn new(db: PgPool) -> Self {
        let creature_repo = CreatureRepository::new(db.clone());
        let session_repo = SessionRepository::new(db.clone());
        let init_group_repo = InitGroupRepository::new(db);
        Self {
            creature_repo,
            session_repo,
            init_group_repo,
        }
    }

    pub async fn create_creature(
        &self,
        session_id: &str,
        secret: &str,
        init_group_id: &str,
        creature_name: &str,
        max_hp: i32,
        curr_hp: i32,
        hp_hidden: bool,
        icon: Option<String>,
    ) -> DomainResult<CreatureEntity> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let init_group_id = Uuid::parse_str(init_group_id).map_err(DomainError::InvalidUuid)?;
        let _session = self
            .session_repo
            .find_by_id_and_secret(&session_id, &secret)
            .await?;
        let creature = CreatureEntity {
            rpghp_creature_id: Uuid::new_v4(),
            creature_name: String::from(creature_name),
            max_hp,
            curr_hp,
            hp_hidden,
            icon,
            init_group_id,
        };
        self.creature_repo.create(&creature).await?;

        Ok(creature)
    }

    pub async fn get_all_creatures_for_session(
        &self,
        session_id: &str,
        opt_secret: Option<&String>,
    ) -> DomainResult<Vec<CreatureEntity>> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let session = match opt_secret {
            None => self.session_repo.find_by_id(&session_id).await?,
            Some(token) => {
                let token = Uuid::parse_str(token).map_err(DomainError::InvalidUuid)?;
                self.session_repo
                    .find_by_id_and_secret(&session_id, &token)
                    .await?
            }
        };

        let Ok(creatures) = self
            .creature_repo
            .find_by_session_id(&session.rpghp_session_id)
            .await
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
    ) -> DomainResult<CreatureEntity> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let creature_id = Uuid::parse_str(creature_id).map_err(DomainError::InvalidUuid)?;
        let session = match opt_secret {
            None => self.session_repo.find_by_id(&session_id).await?,
            Some(token) => {
                let token = Uuid::parse_str(token).map_err(DomainError::InvalidUuid)?;
                self.session_repo
                    .find_by_id_and_secret(&session_id, &token)
                    .await?
            }
        };
        let creature = self.creature_repo.find_by_id(&creature_id).await?;
        let init_group = self
            .init_group_repo
            .find_by_id(creature.init_group_id)
            .await?;
        if init_group.session_id != session.rpghp_session_id {
            return Err(DomainError::Forbidden);
        }

        Ok(creature)
    }
}
