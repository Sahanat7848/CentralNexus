use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    entities::brawlers::BrawlerEntity,
    entities::friendships::NewFriendshipEntity,
    repositories::{brawlers::BrawlerRepository, friendships::FriendshipRepository},
};

pub struct FriendshipUseCase<FR, BR>
where
    FR: FriendshipRepository + Send + Sync,
    BR: BrawlerRepository + Send + Sync,
{
    friendship_repository: Arc<FR>,
    brawler_repository: Arc<BR>,
}

impl<FR, BR> FriendshipUseCase<FR, BR>
where
    FR: FriendshipRepository + Send + Sync,
    BR: BrawlerRepository + Send + Sync,
{
    pub fn new(friendship_repository: Arc<FR>, brawler_repository: Arc<BR>) -> Self {
        Self {
            friendship_repository,
            brawler_repository,
        }
    }

    pub async fn search_friend(&self, query: String) -> Result<BrawlerEntity> {
        // query format: "Name#1234"
        let parts: Vec<&str> = query.split('#').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid search format. Use Name#1234"));
        }

        let name = parts[0].trim();
        let tag = parts[1].trim();

        let friend = self
            .brawler_repository
            .find_by_name_and_tag(name, tag)
            .await?;

        match friend {
            Some(f) => Ok(f),
            None => Err(anyhow::anyhow!("Brawler not found")),
        }
    }

    pub async fn add_friend(&self, brawler_id: i32, friend_id: i32) -> Result<()> {
        if brawler_id == friend_id {
            return Err(anyhow::anyhow!("You cannot add yourself as a friend"));
        }

        // Check if already friends
        let existing = self
            .friendship_repository
            .find_friendship(brawler_id, friend_id)
            .await?;
        if existing.is_some() {
            return Err(anyhow::anyhow!("Already friends"));
        }

        let friendship = NewFriendshipEntity {
            brawler_id,
            friend_id,
            status: "accepted".to_string(), // Direct add for simplicity
        };

        self.friendship_repository.add_friend(friendship).await?;

        // Bidirectional friendship
        let reverse_friendship = NewFriendshipEntity {
            brawler_id: friend_id,
            friend_id: brawler_id,
            status: "accepted".to_string(),
        };
        self.friendship_repository
            .add_friend(reverse_friendship)
            .await?;

        Ok(())
    }

    pub async fn get_friends(&self, brawler_id: i32) -> Result<Vec<BrawlerEntity>> {
        self.friendship_repository.get_friends(brawler_id).await
    }
}
