use app::App;

pub mod app;
pub mod components;
pub mod messages;
pub mod views;

pub fn main() -> iced::Result {
    App::run()
}
