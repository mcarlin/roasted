use crate::bean::model::Bean;
use crate::AppState;
use axum::extract::{Path as AxumPath, State};
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use utoipa::Path;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/beans",
    responses(
        (status = 200, description = "List all beans successfully", body = [Bean])
    )
)]
async fn list_beans_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Bean>>, (StatusCode, String)> {
    match state.bean_service.get_beans().await {
        Ok(beans) => Ok(Json(beans)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn create_bean_handler(
    State(state): State<AppState>,
    Json(bean): Json<Bean>,
) -> Result<Json<Bean>, (StatusCode, String)> {
    match state.bean_service.create(bean).await {
        Ok(bean) => Ok(Json(bean)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_bean_handler(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<Uuid>,
    Json(bean): Json<Bean>,
) -> Result<Json<Bean>, (StatusCode, String)> {
    match state.bean_service.update(id, bean).await {
        Ok(bean) => Ok(Json(bean)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn bean_handler(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<Uuid>,
) -> Result<Json<Bean>, (StatusCode, String)> {
    match state.bean_service.get_bean_by_id(id).await {
        Ok(beans) => Ok(Json(beans)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn bean_routes(x: &AppState) -> Router<()> {
    Router::new()
        .route("/beans", get(list_beans_handler))
        .route("/beans", post(create_bean_handler))
        .route("/beans/:id", put(update_bean_handler))
        .route("/beans/:id", get(bean_handler))
        .with_state(x.clone())
}

pub fn openapi() -> Vec<(String, utoipa::openapi::path::PathItem)> {
    vec![(
        __path_list_beans_handler::path(),
        __path_list_beans_handler::path_item(None),
    )]
}
