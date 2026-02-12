use crate::domain::entities::messages::Message;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn send_message(
        &self,
        sender_id: i32,
        receiver_id: i32,
        content: String,
    ) -> Result<Message>;
    async fn get_conversation(&self, brawler_id: i32, friend_id: i32) -> Result<Vec<Message>>;
    async fn mark_as_read(&self, receiver_id: i32, sender_id: i32) -> Result<()>;
}
