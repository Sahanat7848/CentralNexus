use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    constants::MAX_CREW_PER_MISSION,
    repositories::{
        mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
    },
    value_object::mission_statuses::MissionStatuses,
};

pub struct MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    mission_operation_repository: Arc<T1>,
    missiom_viewing_repository: Arc<T2>,
}

impl<T1, T2> MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(mission_operation_repository: Arc<T1>, missiom_viewing_repository: Arc<T2>) -> Self {
        Self {
            mission_operation_repository,
            missiom_viewing_repository,
        }
    }
    pub async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self
            .missiom_viewing_repository
            .view_detail(mission_id)
            .await?;

        let crew_count = self
            .missiom_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let is_status_open_or_fail = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();

        if !is_status_open_or_fail {
            return Err(anyhow::anyhow!(
                "Mission status must be 'Open' or 'Failed' to start (current: {})",
                mission.status
            ));
        }

        if crew_count <= 0 {
            return Err(anyhow::anyhow!(
                "Mission must have at least one crew member to start"
            ));
        }

        if crew_count > MAX_CREW_PER_MISSION as u32 {
            return Err(anyhow::anyhow!(
                "Mission crew exceeds maximum limit of {}",
                MAX_CREW_PER_MISSION
            ));
        }

        if mission.chief_id != chief_id {
            return Err(anyhow::anyhow!(
                "Only the Chief (ID: {}) can start this mission",
                mission.chief_id
            ));
        }

        let result = self
            .mission_operation_repository
            .in_progress(mission_id, chief_id)
            .await?;
        Ok(result)
    }
    pub async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self
            .missiom_viewing_repository
            .view_detail(mission_id)
            .await?;

        if mission.status != MissionStatuses::InProgress.to_string() {
            return Err(anyhow::anyhow!(
                "Mission must be 'InProgress' to complete (current: {})",
                mission.status
            ));
        }

        if mission.chief_id != chief_id {
            return Err(anyhow::anyhow!(
                "Only the Chief (ID: {}) can complete this mission",
                mission.chief_id
            ));
        }

        let result = self
            .mission_operation_repository
            .to_completed(mission_id, chief_id)
            .await?;

        Ok(result)
    }

    pub async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self
            .missiom_viewing_repository
            .view_detail(mission_id)
            .await?;

        if mission.status != MissionStatuses::InProgress.to_string() {
            return Err(anyhow::anyhow!(
                "Mission must be 'InProgress' to fail (current: {})",
                mission.status
            ));
        }

        if mission.chief_id != chief_id {
            return Err(anyhow::anyhow!(
                "Only the Chief (ID: {}) can fail this mission",
                mission.chief_id
            ));
        }

        let result = self
            .mission_operation_repository
            .to_failed(mission_id, chief_id)
            .await?;

        Ok(result)
    }
}
