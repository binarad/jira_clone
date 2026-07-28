use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct Project {
    #[arg(skip)]
    pub id: i32,
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub owner_id: i32,
    #[arg(skip)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
