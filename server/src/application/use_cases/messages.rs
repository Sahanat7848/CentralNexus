use crate::domain::entities::messages::Message;
use crate::domain::repositories::messages::MessageRepository;
use anyhow::Result;
use std::sync::Arc;

pub struct MessageUseCase<R: MessageRepository> {
    repository: Arc<R>,
}

impl<R: MessageRepository> MessageUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn send_message(
        &self,
        sender_id: i32,
        receiver_id: i32,
        content: String,
    ) -> Result<Message> {
        self.repository
            .send_message(sender_id, receiver_id, content)
            .await
    }

    pub async fn get_conversation(&self, brawler_id: i32, friend_id: i32) -> Result<Vec<Message>> {
        // Mark as read when viewing conversation
        let _ = self.repository.mark_as_read(brawler_id, friend_id).await;
        self.repository
            .get_conversation(brawler_id, friend_id)
            .await
    }
}
