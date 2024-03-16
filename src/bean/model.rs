use serde::Serialize;

#[derive(Serialize)]
pub struct Bean {
    pub bean_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub ts: time::OffsetDateTime,
    pub region: Option<String>,
    pub grade: Option<String>,
}

