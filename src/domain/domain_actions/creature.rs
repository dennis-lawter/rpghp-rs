use uuid::Uuid;

use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::domain::records::Record;
use crate::domain::records::creature::CreatureRecord;
use crate::domain::records::session::SessionRecord;

use super::Domain;

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
    ) -> DomainResult<CreatureRecord> {
        let id = Uuid::parse_str(id).map_err(DomainError::InvalidUuid)?;
        let secret = Uuid::parse_str(secret).map_err(DomainError::InvalidUuid)?;
        let session = SessionRecord::find_by_id_and_secret(&self.db, &id, &secret).await?;
        let creature = CreatureRecord {
            rpghp_creature_id: Uuid::new_v4(),
            session_id: session.rpghp_session_id,
            creature_name: String::from(creature_name),
            max_hp,
            curr_hp,
            hp_hidden,
            icon,
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
