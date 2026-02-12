use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // จำกัดให้มีแค่ 2 การเชื่อมต่อพร้อมกัน (สำหรับตัวฟรี Supabase)
    let pool = Pool::builder()
        .max_size(2) 
        .build(manager)?;
    Ok(pool)
}