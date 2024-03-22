use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Bean {
    pub bean_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ts: time::OffsetDateTime,
    pub region: Option<String>,
    pub grade: Option<String>,
}
