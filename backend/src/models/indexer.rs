use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::{DateTime, Local};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Indexer {
    pub id: i32,
    pub name: String,
    pub auth_data: Value,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewIndexer {
    pub name: String,
    pub auth_data: Value,
}
