use crate::config::DatabaseConfig;
use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

pub async fn create_pool(config: &DatabaseConfig) -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(config.user.clone());
    cfg.password = Some(config.password.clone());
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database.clone());
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}
