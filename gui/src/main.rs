use anyhow::{Context, Result};
use app::App;
use sqlx::PgPool;

pub mod app;
pub mod components;
pub mod messages;
pub mod views;

pub fn main() -> iced::Result {
    App::run()
}
