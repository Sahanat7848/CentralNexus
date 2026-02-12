use std::sync::Arc;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    domain::{
        repositories::mission_viewing::MissionViewingRepository,
        value_object::{
            brawler_model::BrawlerModel, mission_filter::MissionFilter,
            mission_moddel::MissionModel,
        },
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::crew_memberships},
};
pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MissionViewingPostgres {
    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let value = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .first::<i64>(&mut conn)?;

        let count = u32::try_from(value)?;
        Ok(count)
    }

    async fn view_detail(&self, mission_id: i32) -> Result<MissionModel> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let sql = r#"
            SELECT 
                m.id, 
                m.name, 
                m.description, 
                m.status, 
                m.chief_id, 
                b.display_name as chief_display_name,
                COALESCE(cc.crew_count, 0) as crew_count,
                m.created_at, 
                m.updated_at
            FROM missions m
            INNER JOIN brawlers b ON b.id = m.chief_id
            LEFT JOIN (
                SELECT mission_id, COUNT(*) as crew_count 
                FROM crew_memberships 
                GROUP BY mission_id
            ) cc ON cc.mission_id = m.id
            WHERE m.id = $1 AND m.deleted_at IS NULL
        "#;

        let result = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(mission_id)
            .get_result::<MissionModel>(&mut conn)?;

        Ok(result)
    }

    async fn gets(&self, mission_filter: &MissionFilter) -> Result<Vec<MissionModel>> {
        use diesel::sql_types::{Nullable, Varchar};

        let mut conn = Arc::clone(&self.db_pool).get()?;

        let sql = r#"
            SELECT 
                m.id, 
                m.name, 
                m.description, 
                m.status, 
                m.chief_id, 
                b.display_name as chief_display_name,
                COALESCE(cc.crew_count, 0) as crew_count,
                m.created_at, 
                m.updated_at
            FROM missions m
            INNER JOIN brawlers b ON b.id = m.chief_id
            LEFT JOIN (
                SELECT mission_id, COUNT(*) as crew_count 
                FROM crew_memberships 
                GROUP BY mission_id
            ) cc ON cc.mission_id = m.id
            WHERE 
                m.deleted_at IS NULL AND
                ($1 IS NULL OR m.status = $1) AND
                ($2 IS NULL OR m.name ILIKE $2) AND
                ($3 IS NULL OR EXISTS (
                    SELECT 1 FROM crew_memberships cm 
                    WHERE cm.mission_id = m.id AND cm.brawler_id = $3
                ))
            ORDER BY m.created_at DESC
        "#;

        let status_bind: Option<String> = mission_filter.status.as_ref().map(|s| s.to_string());
        let name_bind: Option<String> = mission_filter.name.as_ref().map(|n| format!("%{}%", n));
        let brawler_id_bind = mission_filter.brawler_id;

        let rows = diesel::sql_query(sql)
            .bind::<Nullable<Varchar>, _>(status_bind)
            .bind::<Nullable<Varchar>, _>(name_bind)
            .bind::<Nullable<diesel::sql_types::Int4>, _>(brawler_id_bind)
            .load::<MissionModel>(&mut conn)?;

        Ok(rows)
    }
    async fn get_mission_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let sql = r#"
            SELECT 
               b.display_name AS display_name,
               COALESCE(b.avatar_url, '') AS avatar_url,
               COALESCE(s.success_count, 0) AS mission_success_count,
               COALESCE(j.joined_count, 0) AS mission_joined_count
            FROM 
                crew_memberships cm
            INNER JOIN 
                brawlers b ON b.id = cm.brawler_id 
            LEFT JOIN 
                (
                    SELECT 
                        cm2.brawler_id, 
                        COUNT(*) AS success_count
                    FROM 
                        crew_memberships cm2
                    INNER JOIN 
                        missions m2 ON m2.id = cm2.mission_id
                    WHERE 
                        m2.status = 'completed'
                    GROUP BY 
                        cm2.brawler_id
                ) s ON s.brawler_id = cm.brawler_id
            LEFT JOIN 
                (
                    SELECT 
                        cm3.brawler_id, 
                        COUNT(*) AS joined_count
                    FROM 
                        crew_memberships cm3
                    GROUP BY 
                        cm3.brawler_id
                ) j ON j.brawler_id = b.id
            WHERE 
                cm.mission_id = $1
        "#;

        let result = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(mission_id)
            .load::<BrawlerModel>(&mut conn)?;

        Ok(result)
    }
}
