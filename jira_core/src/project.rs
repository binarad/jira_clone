use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub owner_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
