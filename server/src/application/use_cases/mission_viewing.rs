use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::mission_viewing::MissionViewingRepository,
    value_object::{
        brawler_model::BrawlerModel, mission_filter::MissionFilter, mission_moddel::MissionModel,
    },
};
pub struct MissionViewingUseCase<T>
where
    T: MissionViewingRepository + Send + Sync,
{
    mission_viewing_repository: Arc<T>,
}

impl<T> MissionViewingUseCase<T>
where
    T: MissionViewingRepository + Send + Sync,
{
    pub fn new(mission_viewing_repository: Arc<T>) -> Self {
        Self {
            mission_viewing_repository,
        }
    }

    pub async fn get_one(&self, mission_id: i32) -> Result<MissionModel> {
        let result = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        Ok(result)
    }
    pub async fn get_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let result = self
            .mission_viewing_repository
            .get_mission_crew(mission_id)
            .await?;

        Ok(result)
    }

    pub async fn get_all(&self, filter: &MissionFilter) -> Result<Vec<MissionModel>> {
        let result = self.mission_viewing_repository.gets(filter).await?;

        Ok(result)
    }
}
