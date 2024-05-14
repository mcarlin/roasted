use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Bean {
    #[schema(value_type = String, format = "uuid")]
    pub bean_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String, format = "date-time")]
    pub ts: time::OffsetDateTime,
    pub region: Option<String>,
    pub grade: Option<String>,
}
