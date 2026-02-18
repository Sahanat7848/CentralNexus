use crate::domain::{
    entities::crew_memberships::CrewMemberShips,
    repositories::{
        crew_oparation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
    },
};
use anyhow::Result;
use std::sync::Arc;

pub struct CrewOperationUseCase<T1, T2>
where
    T1: CrewOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    crew_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewOperationUseCase<T1, T2>
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(crew_operation_repository: Arc<T1>, mission_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_operation_repository,
            mission_viewing_repository,
        }
    }

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let max_crew_per_mission = std::env::var("MAX_CREW_PER_MISSION")
            .expect("missing value")
            .parse()?;

        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        if mission.chief_id == brawler_id {
            return Err(anyhow::anyhow!(
                "Chiefs cannot join their own missions as crew members!!"
            ));
        }

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        tracing::info!(
            "Brawler({}) attempting to join Mission({}). Status: {}, Crew count: {}",
            brawler_id,
            mission_id,
            mission.status,
            crew_count
        );

        let status_lower = mission.status.to_lowercase();
        let mission_status_condition = status_lower == "open" || status_lower == "failed";

        if !mission_status_condition {
            tracing::warn!(
                "Mission({}) status '{}' is not joinable",
                mission_id,
                mission.status
            );
            return Err(anyhow::anyhow!(
                "Mission is not joinable in current status: {}",
                mission.status
            ));
        }
        let crew_count_condition = crew_count < max_crew_per_mission;
        if !crew_count_condition {
            tracing::warn!(
                "Mission({}) is full ({} >= {})",
                mission_id,
                crew_count,
                max_crew_per_mission
            );
            return Err(anyhow::anyhow!("Mission is full"));
        }

        self.crew_operation_repository
            .join(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }

    pub async fn leave(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        tracing::info!(
            "Brawler({}) attempting to leave Mission({}). Status: {}",
            brawler_id,
            mission_id,
            mission.status
        );

        let status_lower = mission.status.to_lowercase();
        let leaving_condition =
            status_lower == "open" || status_lower == "failed" || status_lower == "inprogress";

        if !leaving_condition {
            tracing::warn!(
                "Mission({}) status '{}' is not leavable",
                mission_id,
                mission.status
            );
            return Err(anyhow::anyhow!(
                "Mission is not leavable in current status: {}",
                mission.status
            ));
        }
        self.crew_operation_repository
            .leave(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }
}
