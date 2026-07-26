use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: String,          // Todo create UserRole struct
    pub email: String,         // Todo email validation later?
    pub password_hash: String, // ?
    pub created_at: chrono::DateTime<chrono::Utc>,
}
