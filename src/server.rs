use crate::bean::routes::bean_routes;
use crate::roast::routes::roast_routes;
use crate::AppState;
use axum::http::StatusCode;
use axum::Router;
use std::error::Error;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn serve(app_state: AppState) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .with_state(app_state.clone())
        .nest(
            "/v1",
            Router::new()
                .merge(bean_routes(&app_state))
                .merge(roast_routes(&app_state)),
        )
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await.map_err(|e| e.into())
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
