use crate::bean::routes::bean_routes;
use crate::config::{RoastedConfig, ServerConfig};
use crate::roast::routes::roast_routes;
use crate::{bean, run_application, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use std::error::Error;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::openapi::{OpenApiBuilder, ServerBuilder};
use utoipa::{openapi, ToSchema};

#[derive(Clone, Debug)]
pub struct ServerState {
    addr: String,
}

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

    let listener = tokio::net::TcpListener::bind(app_state.server_state.addr.clone()).await?;

    info!("Serving Roasted @ {}", app_state.server_state.addr);
    axum::serve(listener, app).await.map_err(|e| e.into())
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
async fn openapi(State(state): State<AppState>) -> Json<openapi::OpenApi> {
    let mut builder = OpenApiBuilder::new();

    let mut info_builder = openapi::InfoBuilder::new();
    info_builder = info_builder.title("Roasted");
    info_builder = info_builder.description(Some("Coffee roasting and analytics"));
    info_builder = info_builder.version(VERSION);
    builder = builder.info(info_builder.build());

    let mut server_builder = ServerBuilder::new();
    server_builder = server_builder.description(Some(
        "API to Roasted, the coffee roasting and analytics app".to_string(),
    ));
    server_builder = server_builder.url(state.server_state.addr.clone());
    builder = builder.servers(Some(vec![server_builder.build()]));

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

pub(crate) fn build_state(config: &RoastedConfig) -> ServerState {
    let addr = format!("0.0.0.0:{}", config.server.port);

    ServerState { addr }
}
