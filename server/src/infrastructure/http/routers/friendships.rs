use crate::{
    application::use_cases::friendships::FriendshipUseCase,
    domain::value_object::brawler_model::BrawlerSummaryModel,
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{brawlers::BrawlerPostgres, friendships::FriendshipPostgres},
        },
        http::middlewares::auth::authorization,
    },
};
use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let friendship_repository = FriendshipPostgres::new(Arc::clone(&db_pool));
    let brawler_repository = BrawlerPostgres::new(db_pool);
    let friendship_use_case = FriendshipUseCase::new(
        Arc::new(friendship_repository),
        Arc::new(brawler_repository),
    );

    Router::new()
        .route("/friends/search", get(search_friend))
        .route("/friends/add", post(add_friend))
        .route("/friends", get(get_friends))
        .layer(axum::middleware::from_fn(authorization))
        .with_state(Arc::new(friendship_use_case))
}

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub query: String, // "Name#1234"
}

pub async fn search_friend(
    State(friendship_use_case): State<Arc<FriendshipUseCase<FriendshipPostgres, BrawlerPostgres>>>,
    Query(search_query): Query<SearchQuery>,
) -> impl IntoResponse {
    match friendship_use_case.search_friend(search_query.query).await {
        Ok(friend) => (StatusCode::OK, Json(BrawlerSummaryModel::from(friend))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct AddFriendRequest {
    pub friend_id: i32,
}

pub async fn add_friend(
    State(friendship_use_case): State<Arc<FriendshipUseCase<FriendshipPostgres, BrawlerPostgres>>>,
    Extension(brawler_id): Extension<i32>,
    Json(add_friend_request): Json<AddFriendRequest>,
) -> impl IntoResponse {
    match friendship_use_case
        .add_friend(brawler_id, add_friend_request.friend_id)
        .await
    {
        Ok(_) => (StatusCode::OK, "Friend added successfully").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_friends(
    State(friendship_use_case): State<Arc<FriendshipUseCase<FriendshipPostgres, BrawlerPostgres>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse {
    match friendship_use_case.get_friends(brawler_id).await {
        Ok(friends) => {
            let friends_summary: Vec<BrawlerSummaryModel> =
                friends.into_iter().map(BrawlerSummaryModel::from).collect();
            (StatusCode::OK, Json(friends_summary)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
