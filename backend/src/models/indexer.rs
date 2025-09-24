use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthItem {
    pub value: String,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Indexer {
    pub id: i32,
    pub name: String,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewIndexer {
    pub name: String,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
}
