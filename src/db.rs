use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;
use crate::config::{DatabaseConfig};

pub async fn create_pool(config: DatabaseConfig) -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(config.user);
    cfg.password = Some(config.password);
    cfg.host = Some(config.host);
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database);
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}