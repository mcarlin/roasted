use crate::bean::routes::bean_routes;
use crate::roast::routes::roast_routes;
use crate::{bean, AppState};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use std::error::Error;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::openapi::OpenApiBuilder;
use utoipa::{openapi, ToSchema};

pub async fn serve(app_state: AppState) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/api-docs/openapi.json", get(openapi))
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

async fn openapi() -> Json<openapi::OpenApi> {
    let mut builder = OpenApiBuilder::new();

    let mut paths = Vec::new();
    paths.append(&mut bean::routes::openapi());
    let mut paths_builder = openapi::path::PathsBuilder::new();
    for (path, item) in paths {
        paths_builder = paths_builder.path(path, item);
    }

    let schemas = vec![bean::model::Bean::schema()];
    let mut components_builder = openapi::schema::ComponentsBuilder::new();
    for (name, schema) in schemas.into_iter() {
        components_builder = components_builder.schema(name, schema);
    }

    builder = builder.components(Some(components_builder.build()));
    builder = builder.paths(paths_builder.build());

    Json(builder.build())
}
