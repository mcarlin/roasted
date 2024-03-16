use std::env;
use std::error::Error;
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoastedConfig {
    pub db: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

pub(crate) async fn read_config() -> Result<RoastedConfig, Box<dyn Error>> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
    let s = Config::builder()
        .add_source(File::with_name("config/default.yaml"))
        .add_source(
            File::with_name(&format!("config/{}.yaml", run_mode))
                .required(false),
        )
        .add_source(File::with_name("config/local.yaml").required(false))
        .add_source(Environment::with_prefix("roasted"))
        .build()?;
    s.try_deserialize()
        .map_err(|e| e.into())
}