use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Bean {
    pub bean_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ts: time::OffsetDateTime,
    pub region: Option<String>,
    pub grade: Option<String>,
}
