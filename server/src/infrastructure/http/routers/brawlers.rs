use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_object::{brawler_model::RegisterBrawlerModel, upload_image::UploadAvatar},
    },
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        http::middlewares::auth::authorization,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawlers_repository = BrawlerPostgres::new(db_pool);
    let brawlers_use_case = BrawlersUseCase::new(Arc::new(brawlers_repository));

    let protected_router = Router::new()
        .route("/avatar", post(upload_avatar))
        .route("/my-missions", get(get_mission))
        .route("/update-name", post(update_name))
        .route_layer(axum::middleware::from_fn(authorization));

    Router::new()
        .route("/register", post(register))
        .merge(protected_router)
        .with_state(Arc::new(brawlers_use_case))
}

pub async fn get_mission<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.get_my_missions(brawler_id).await {
        Ok(missions) => (StatusCode::OK, Json(missions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn register<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.register(register_brawler_model).await {
        Ok(passport) => (StatusCode::CREATED, Json(passport)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn upload_avatar<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_image): Json<UploadAvatar>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .upload_base64image(brawler_id, upload_image.base64_string)
        .await
    {
        Ok(uploaded_image) => (StatusCode::CREATED, Json(uploaded_image)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateNameModel {
    pub display_name: String,
}

pub async fn update_name<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(update_name_model): Json<UpdateNameModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .update_display_name(brawler_id, update_name_model.display_name)
        .await
    {
        Ok(_) => (StatusCode::OK, "Name updated successfully").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
