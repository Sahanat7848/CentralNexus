use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    insert_into,
};
use std::sync::Arc;

use crate::{
    domain::{
        entities::{
            brawlers::BrawlerEntity,
            friendships::{FriendshipEntity, NewFriendshipEntity},
        },
        repositories::friendships::FriendshipRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{brawlers, friendships},
    },
};

pub struct FriendshipPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl FriendshipPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl FriendshipRepository for FriendshipPostgres {
    async fn add_friend(&self, friendship: NewFriendshipEntity) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        insert_into(friendships::table)
            .values(&friendship)
            .execute(&mut connection)?;

        Ok(())
    }

    async fn find_friendship(
        &self,
        brawler_id: i32,
        friend_id: i32,
    ) -> Result<Option<FriendshipEntity>> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = friendships::table
            .filter(friendships::brawler_id.eq(brawler_id))
            .filter(friendships::friend_id.eq(friend_id))
            .select(FriendshipEntity::as_select())
            .first::<FriendshipEntity>(&mut connection)
            .optional()?;

        Ok(result)
    }

    async fn get_friends(&self, brawler_id: i32) -> Result<Vec<BrawlerEntity>> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let friends = brawlers::table
            .inner_join(friendships::table.on(friendships::friend_id.eq(brawlers::id)))
            .filter(friendships::brawler_id.eq(brawler_id))
            // .filter(friendships::status.eq("accepted")) // For now, we'll just get all contacts if simple add friend
            .select(BrawlerEntity::as_select())
            .load::<BrawlerEntity>(&mut connection)?;

        Ok(friends)
    }
}
