use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, Args)]
pub struct User {
    #[arg(skip)]
    pub id: Option<i32>,
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub role: String, // Todo create UserRole struct
    #[arg(short, long)]
    pub email: String, // Todo email validation later?
    #[arg(short, long)]
    pub password_hash: String, // ?
    #[arg(skip)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
