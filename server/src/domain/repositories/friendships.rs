use crate::domain::entities::brawlers::BrawlerEntity;
use crate::domain::entities::friendships::{FriendshipEntity, NewFriendshipEntity};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait FriendshipRepository {
    async fn add_friend(&self, friendship: NewFriendshipEntity) -> Result<()>;
    async fn find_friendship(
        &self,
        brawler_id: i32,
        friend_id: i32,
    ) -> Result<Option<FriendshipEntity>>;
    async fn get_friends(&self, brawler_id: i32) -> Result<Vec<BrawlerEntity>>;
}
