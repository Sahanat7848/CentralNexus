use crate::{
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_object::{
            base64_image::Base64Image, brawler_model::RegisterBrawlerModel,
            mission_moddel::MissionModel, upload_image::UploadedImage,
        },
    },
    infrastructure::{argon2::hash, cloudinary::UploadImageOptions, jwt::jwt_model::Passport},
};
use anyhow::Result;
use std::sync::Arc;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn register(&self, mut register_model: RegisterBrawlerModel) -> Result<Passport> {
        let hashed_password = hash(register_model.password.clone())?;

        register_model.password = hashed_password;

        let register_entity = register_model.to_entity();
        let tag = register_entity.tag.clone();

        let user_id = self.brawler_repository.register(register_entity).await?;

        let passport = Passport::new(user_id, register_model.display_name, tag, None)?;
        Ok(passport)
    }
    pub async fn upload_base64image(
        &self,
        user_id: i32,
        base64_image: String,
    ) -> Result<UploadedImage> {
        let opt = UploadImageOptions {
            folder: Some("avatars".to_string()),
            public_id: Some(user_id.to_string()),
            transformation: Some("c_scale,w_256".to_string()),
        };
        let base64_image_vo = Base64Image::new(base64_image)?;
        let uploaded_image = self
            .brawler_repository
            .upload_base64image(user_id, base64_image_vo, opt)
            .await?;

        Ok(uploaded_image)
    }

    pub async fn update_display_name(&self, brawler_id: i32, new_name: String) -> Result<()> {
        self.brawler_repository
            .update_name(brawler_id, new_name)
            .await?;
        Ok(())
    }

    pub async fn get_my_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        let result = self.brawler_repository.get_missions(brawler_id).await?;

        Ok(result)
    }
}
