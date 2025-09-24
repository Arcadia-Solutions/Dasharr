use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Indexer {
    pub id: i32,
    pub name: String,
    pub api_key: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewIndexer {
    pub name: String,
    pub api_key: Option<String>,
}
