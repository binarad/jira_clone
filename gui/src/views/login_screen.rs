use crate::messages::Message;
use iced::Element;
use iced::widget::{container, text};

fn login_screen_view() -> Element<'_, Message> {
    let content = text("Login Screen!");
    container(content).into()
}
