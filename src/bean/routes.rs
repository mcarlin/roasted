use crate::bean::model::Bean;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;

async fn beans_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Bean>>, (StatusCode, String)> {
    match state.bean_service.get_beans().await {
        Ok(beans) => Ok(Json(beans)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn bean_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Bean>, (StatusCode, String)> {
    match state.bean_service.get_bean_by_id(id).await {
        Ok(beans) => Ok(Json(beans)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn bean_routes(x: &AppState) -> Router<()> {
    Router::new()
        .route("/beans", get(beans_handler))
        .route("/beans/:id", get(bean_handler))
        .with_state(x.clone())
}
