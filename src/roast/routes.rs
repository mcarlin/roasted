use crate::cornucopia::queries::roast::update_roast_step;
use crate::roast::model::{Roast, RoastLevel, RoastStep};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use uuid::Uuid;

async fn get_roasts_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Roast>>, (StatusCode, String)> {
    match state.roast_service.get_all_roasts().await {
        Ok(roasts) => Ok(Json(roasts)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_roast_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Roast>, (StatusCode, String)> {
    match state.roast_service.get_roast_by_id(id).await {
        Ok(Some(roast)) => Ok(Json(roast)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn create_roast_handler(
    State(state): State<AppState>,
    Json(roast): Json<Roast>,
) -> Result<Json<Roast>, (StatusCode, String)> {
    match state.roast_service.create_roast(roast).await {
        Ok(roast) => Ok(Json(roast)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_roast_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(roast): Json<Roast>,
) -> Result<Json<Roast>, (StatusCode, String)> {
    match state.roast_service.update_roast(id, roast).await {
        Ok(roast) => Ok(Json(roast)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_roast_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), (StatusCode, String)> {
    match state.roast_service.delete_roast(id).await {
        Ok(_) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_roast_steps_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<RoastStep>>, (StatusCode, String)> {
    match state.roast_service.get_all_roast_steps(id).await {
        Ok(steps) => Ok(Json(steps)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_roast_step_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<RoastStep>>, (StatusCode, String)> {
    match state.roast_service.get_all_roast_steps(id).await {
        Ok(steps) => Ok(Json(steps)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn create_roast_step_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(mut step): Json<RoastStep>,
) -> Result<Json<RoastStep>, (StatusCode, String)> {
    step.roast_step_id = id; // path wins

    match state.roast_service.create_roast_step(step).await {
        Ok(rs) => Ok(Json(rs)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_roast_step_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(mut step): Json<RoastStep>,
) -> Result<Json<RoastStep>, (StatusCode, String)> {
    step.roast_step_id = id; // path wins

    match state.roast_service.update_roast_step(step).await {
        Ok(rs) => Ok(Json(rs)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_roast_step_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), (StatusCode, String)> {
    match state.roast_service.delete_roast_step(id).await {
        Ok(_) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_roast_levels_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<RoastLevel>>, (StatusCode, String)> {
    match state.roast_service.get_all_roast_levels().await {
        Ok(levels) => Ok(Json(levels)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn roast_routes(x: &AppState) -> Router<()> {
    Router::new()
        .route("/roasts", get(get_roasts_handler))
        .route("/roasts", post(create_roast_handler))
        .route("/roasts", delete(delete_roast_handler))
        .route("/roasts/:id", put(update_roast_handler))
        .route("/roasts/:id", get(get_roast_handler))
        .route("/roasts/:id/steps", get(get_roast_steps_handler))
        .route("/roasts/:id/steps", post(create_roast_step_handler))
        .route("/roast-steps/:id", get(get_roast_step_handler))
        .route("/roast-steps/:id", put(update_roast_step_handler))
        .route("/roast-steps/:id", delete(delete_roast_step_handler))
        .route("/roast-levels", get(get_roast_levels_handler))
        .with_state(x.clone())
}
