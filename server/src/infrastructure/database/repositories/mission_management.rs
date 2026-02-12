use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::entities::missions::{AddMissionEntity, EditMissionEntity};
use crate::domain::repositories::mission_management::MissionManagementRepository;
use crate::domain::value_object::mission_statuses::MissionStatuses;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::missions;

pub struct MissionManagementPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionManagementPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionManagementRepository for MissionManagementPostgres {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let result = diesel::insert_into(missions::table)
            .values(&add_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set(&edit_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set((
                missions::deleted_at.eq(diesel::dsl::now),
                missions::chief_id.eq(chief_id),
            ))
            .execute(&mut connection)?;

        Ok(())
    }
}
