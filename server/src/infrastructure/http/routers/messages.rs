use crate::{
    application::use_cases::messages::MessageUseCase,
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::messages::MessagePostgres},
        http::middlewares::auth::authorization,
    },
};
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let message_repository = MessagePostgres::new(db_pool);
    let message_use_case = MessageUseCase::new(Arc::new(message_repository));

    Router::new()
        .route("/messages/{friend_id}", get(get_conversation))
        .route("/messages/send", post(send_message))
        .layer(axum::middleware::from_fn(authorization))
        .with_state(Arc::new(message_use_case))
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub receiver_id: i32,
    pub content: String,
}

pub async fn send_message(
    State(message_use_case): State<Arc<MessageUseCase<MessagePostgres>>>,
    Extension(brawler_id): Extension<i32>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    match message_use_case
        .send_message(brawler_id, payload.receiver_id, payload.content)
        .await
    {
        Ok(msg) => (StatusCode::OK, Json(msg)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_conversation(
    State(message_use_case): State<Arc<MessageUseCase<MessagePostgres>>>,
    Extension(brawler_id): Extension<i32>,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse {
    match message_use_case
        .get_conversation(brawler_id, friend_id)
        .await
    {
        Ok(msgs) => (StatusCode::OK, Json(msgs)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
