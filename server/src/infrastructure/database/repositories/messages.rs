use crate::domain::entities::messages::Message;
use crate::domain::repositories::messages::MessageRepository;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::messages;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::sync::Arc;

pub struct MessagePostgres {
    pool: Arc<PgPoolSquad>,
}

impl MessagePostgres {
    pub fn new(pool: Arc<PgPoolSquad>) -> Self {
        Self { pool }
    }
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = messages)]
pub struct MessageDb {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

impl From<MessageDb> for Message {
    fn from(db: MessageDb) -> Self {
        Message {
            id: db.id,
            sender_id: db.sender_id,
            receiver_id: db.receiver_id,
            content: db.content,
            created_at: db.created_at,
            read_at: db.read_at,
        }
    }
}

#[async_trait]
impl MessageRepository for MessagePostgres {
    async fn send_message(
        &self,
        sender_id: i32,
        receiver_id: i32,
        content: String,
    ) -> Result<Message> {
        let mut conn = self.pool.get().context("Failed to get DB connection")?;

        #[derive(Insertable)]
        #[diesel(table_name = messages)]
        struct NewMessage {
            sender_id: i32,
            receiver_id: i32,
            content: String,
        }

        let new_msg = NewMessage {
            sender_id,
            receiver_id,
            content,
        };

        let result: MessageDb = diesel::insert_into(messages::table)
            .values(&new_msg)
            .get_result(&mut conn)
            .context("Error saving new message")?;

        Ok(Message::from(result))
    }

    async fn get_conversation(&self, brawler_id: i32, friend_id: i32) -> Result<Vec<Message>> {
        let mut conn = self.pool.get().context("Failed to get DB connection")?;

        let results: Vec<MessageDb> = messages::table
            .filter(
                (messages::sender_id
                    .eq(brawler_id)
                    .and(messages::receiver_id.eq(friend_id)))
                .or(messages::sender_id
                    .eq(friend_id)
                    .and(messages::receiver_id.eq(brawler_id))),
            )
            .order(messages::created_at.asc())
            .load::<MessageDb>(&mut conn)
            .context("Error loading conversation")?;

        Ok(results.into_iter().map(Message::from).collect())
    }

    async fn mark_as_read(&self, receiver_id: i32, sender_id: i32) -> Result<()> {
        let mut conn = self.pool.get().context("Failed to get DB connection")?;

        diesel::update(messages::table)
            .filter(messages::receiver_id.eq(receiver_id))
            .filter(messages::sender_id.eq(sender_id))
            .filter(messages::read_at.is_null())
            .set(messages::read_at.eq(diesel::dsl::now))
            .execute(&mut conn)
            .context("Error marking messages as read")?;

        Ok(())
    }
}
