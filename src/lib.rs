use log::info;
use std::error::Error;

use crate::bean::service::BeanService;
use crate::roast::service::RoastService;
use crate::server::ServerState;

mod bean;
mod config;
mod cornucopia;
mod db;
mod roast;
mod server;
mod telemetry;

pub async fn run_application() -> Result<(), Box<dyn Error>> {
    let config = config::read_config().await?;
    telemetry::init_tracing().await;

    info!("Starting roasted");

    let server_state = server::build_state(&config);

    let pool = db::create_pool(&config.db).await?;
    let app_state = AppState {
        server_state,
        bean_service: BeanService {
            db_pool: pool.clone(),
        },
        roast_service: RoastService {
            db_pool: pool.clone(),
        },
    };

    server::serve(app_state, config.server).await
}

#[derive(Clone)]
struct AppState {
    server_state: ServerState,
    bean_service: BeanService,
    roast_service: RoastService,
}
