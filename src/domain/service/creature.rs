use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::entity::creature::CreatureEntity;
use crate::domain::repository::core::RepositoryContext;

#[derive(Clone)]
pub struct CreatureService {
    repos: RepositoryContext,
}
impl CreatureService {
    pub const fn new(repos: RepositoryContext) -> Self {
        Self { repos }
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
            .repos
            .session
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
        self.repos.creature.create(&creature).await?;

        Ok(creature)
    }

    pub async fn get_all_creatures_for_session(
        &self,
        session_id: &str,
        opt_secret: Option<&String>,
    ) -> DomainResult<Vec<CreatureEntity>> {
        let session_id = Uuid::parse_str(session_id).map_err(DomainError::InvalidUuid)?;
        let session = match opt_secret {
            None => self.repos.session.find_by_id(&session_id).await?,
            Some(token) => {
                let token = Uuid::parse_str(token).map_err(DomainError::InvalidUuid)?;
                self.repos
                    .session
                    .find_by_id_and_secret(&session_id, &token)
                    .await?
            }
        };

        let Ok(creatures) = self
            .repos
            .creature
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
            None => self.repos.session.find_by_id(&session_id).await?,
            Some(token) => {
                let token = Uuid::parse_str(token).map_err(DomainError::InvalidUuid)?;
                self.repos
                    .session
                    .find_by_id_and_secret(&session_id, &token)
                    .await?
            }
        };
        let creature = self.repos.creature.find_by_id(&creature_id).await?;
        let init_group = self
            .repos
            .init_group
            .find_by_id(creature.init_group_id)
            .await?;
        if init_group.session_id != session.rpghp_session_id {
            return Err(DomainError::Forbidden);
        }

        Ok(creature)
    }
}
