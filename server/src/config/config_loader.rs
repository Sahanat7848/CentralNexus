use anyhow::Result;

use crate::config::{
    config_model::{CloudinaryEnv, Database, DotEnvyConfig, JwtEnv, Server},
    stage::Stage,
};
use std::str::FromStr;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is valid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is valid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is valid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
    };

    let secret = std::env::var("JWT_SECRET")
        .or_else(|_| std::env::var("JWT_USER_SECRET"))
        .expect("JWT_SECRET or JWT_USER_SECRET not set in .env");

    let config = DotEnvyConfig {
        server,
        database,
        secret,
    };

    Ok(config)
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or_else(|_| "".to_string());

    Stage::from_str(&stage_str).unwrap_or_default()
}

pub fn get_user_secret() -> Result<String> {
    dotenvy::dotenv().ok();
    let secret_env = std::env::var("JWT_SECRET")
        .or_else(|_| std::env::var("JWT_USER_SECRET"))
        .map_err(|_| anyhow::anyhow!("JWT_SECRET or JWT_USER_SECRET not set"))?;
    Ok(secret_env)
}

// pub fn get_user_secret_env() -> Result<JwtEnv> {
pub fn get_jwt_env() -> Result<JwtEnv> {
    dotenvy::dotenv().ok();

    Ok(JwtEnv {
        secret: std::env::var("JWT_USER_SECRET").or_else(|_| std::env::var("JWT_SECRET"))?,
        lift_time_days: std::env::var("JWT_LIFETIME_DAYS")
            .or_else(|_| std::env::var("JTW_LIFTTIME_DAYS"))
            .unwrap_or_else(|_| "7".to_string()) // Default to 7 days
            .parse::<i64>()?,
    })
}

pub fn get_cloudinary_env() -> Result<CloudinaryEnv> {
    dotenvy::dotenv().ok();

    Ok(CloudinaryEnv {
        cloud_name: std::env::var("CLOUDINARY_CLOUD_NAME")?,
        api_key: std::env::var("CLOUDINARY_API_KEY")?,
        api_secret: std::env::var("CLOUDINARY_API_SECRET")?,
    })
}
