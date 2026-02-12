use crate::infrastructure::database::schema::friendships;
use chrono::NaiveDateTime;
use diesel::{Selectable, prelude::*};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = friendships)]
#[diesel(primary_key(brawler_id, friend_id))]
pub struct FriendshipEntity {
    pub brawler_id: i32,
    pub friend_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = friendships)]
pub struct NewFriendshipEntity {
    pub brawler_id: i32,
    pub friend_id: i32,
    pub status: String,
}
