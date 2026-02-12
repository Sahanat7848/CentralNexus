use diesel::{
    prelude::QueryableByName,
    sql_types::{BigInt, Varchar},
};
use serde::{Deserialize, Serialize};

use crate::domain::entities::brawlers::RegisterBrawlerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBrawlerModel {
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub upload_avatar: Option<String>,
}

impl RegisterBrawlerModel {
    pub fn to_entity(&self) -> RegisterBrawlerEntity {
        use rand::Rng;
        let mut rng = rand::rng();
        let tag: u16 = rng.random_range(0..10000);
        let tag_str = format!("{:04}", tag);

        RegisterBrawlerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            display_name: self.display_name.clone(),
            tag: tag_str,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct BrawlerModel {
    #[diesel(sql_type = Varchar)]
    pub display_name: String,
    #[diesel(sql_type = Varchar)]
    pub avatar_url: String,
    #[diesel(sql_type = BigInt)]
    pub mission_success_count: i64,
    #[diesel(sql_type = BigInt)]
    pub mission_joined_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrawlerSummaryModel {
    pub id: i32,
    pub display_name: String,
    pub tag: String,
    pub avatar_url: Option<String>,
}

impl From<crate::domain::entities::brawlers::BrawlerEntity> for BrawlerSummaryModel {
    fn from(entity: crate::domain::entities::brawlers::BrawlerEntity) -> Self {
        Self {
            id: entity.id,
            display_name: entity.display_name,
            tag: entity.tag,
            avatar_url: entity.avatar_url,
        }
    }
}
