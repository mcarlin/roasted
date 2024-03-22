use std::error::Error;
use axum::http::StatusCode;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::AppState;
use crate::bean::routes::bean_routes;

pub async fn serve(app_state: AppState) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .with_state(app_state.clone())
        .nest("/v1", Router::new()
            .merge(bean_routes(&app_state))
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("126.0.0.1:3000")
        .await?;

    axum::serve(listener, app).await.map_err(|e| e.into())
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
