use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper, insert_into,
};
use std::sync::Arc;

use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
        value_object::{
            base64_image::Base64Image, mission_moddel::MissionModel, upload_image::UploadedImage,
        },
    },
    infrastructure::{
        cloudinary::UploadImageOptions,
        database::{
            postgresql_connection::PgPoolSquad,
            schema::{brawlers, crew_memberships},
        },
    },
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: &String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }
    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::id.eq(brawler_id))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_name_and_tag(&self, name: &str, tag: &str) -> Result<Option<BrawlerEntity>> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::display_name.ilike(name))
            .filter(brawlers::tag.eq(tag))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)
            .optional()?;

        Ok(result)
    }

    async fn update_name(&self, brawler_id: i32, new_name: String) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::display_name.eq(new_name),
                brawlers::name_updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn upload_base64image(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage> {
        let uploaded_image =
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::avatar_url.eq(uploaded_image.url.clone()),
                brawlers::avatar_public_id.eq(uploaded_image.public_id.clone()),
            ))
            .execute(&mut conn)?;

        Ok(uploaded_image)
    }

    async fn crew_counting(&self, brawler_id: i32) -> Result<u32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = crew_memberships::table
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .count()
            .get_result::<i64>(&mut connection)?;

        Ok(result as u32)
    }

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
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
            INNER JOIN crew_memberships cm ON cm.mission_id = m.id
            LEFT JOIN (
                SELECT mission_id, COUNT(*) as crew_count 
                FROM crew_memberships 
                GROUP BY mission_id
            ) cc ON cc.mission_id = m.id
            WHERE cm.brawler_id = $1 AND m.deleted_at IS NULL
            ORDER BY m.created_at DESC
        "#;

        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(brawler_id)
            .load::<MissionModel>(&mut conn)?;

        Ok(results)
    }
}
