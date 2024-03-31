use std::error::Error;

use crate::bean::service::BeanService;
use crate::roast::service::RoastService;

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

    let pool = db::create_pool(config.db).await?;

    let app_state = AppState {
        bean_service: BeanService {
            db_pool: pool.clone(),
        },
        roast_service: RoastService {
            db_pool: pool.clone(),
        },
    };

    server::serve(app_state).await
}

#[derive(Clone)]
struct AppState {
    bean_service: BeanService,
    roast_service: RoastService,
}
