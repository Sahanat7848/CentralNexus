use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        value_object::{
            base64_image::Base64Image, mission_moddel::MissionModel, upload_image::UploadedImage,
        },
    },
    infrastructure::cloudinary::UploadImageOptions,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: &String) -> Result<BrawlerEntity>;
    async fn upload_base64image(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage>;
    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity>;
    async fn find_by_name_and_tag(&self, name: &str, tag: &str) -> Result<Option<BrawlerEntity>>;
    async fn update_name(&self, brawler_id: i32, new_name: String) -> Result<()>;
    async fn crew_counting(&self, brawler_id: i32) -> Result<u32>;
    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>>;
}
