use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Roast {
    pub roast_id: Uuid,
    pub bean_id: Uuid,
    pub roast_level_id: Uuid,
    pub ts: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoastLevel {
    pub roast_level_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoastStep {
    pub roast_step_id: Uuid,
    pub roast_id: Uuid,
    pub position: i32,
    pub description: Option<String>,
    pub time: i64,
    pub fan_speed: Option<i32>,
    pub temp_setting: Option<i32>,
    pub temperature: Option<rust_decimal::Decimal>,
}
