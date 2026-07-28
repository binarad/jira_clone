use super::messages::Message;
use db::connect_to_db;
// use anyhow::Result;
use iced::widget::{container, text};
use iced::{Element, Length, Task};
use jira_core::user::User;
use sqlx::PgPool;

pub enum ViewState {
    Loading,
    LoginScreen,
}

pub struct App {
    is_loading: bool,
    status_message: String,
    pool: Option<PgPool>,
    users: Vec<User>,
    current_view: ViewState,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                is_loading: true,
                status_message: "Initializing...".to_string(),
                pool: None,
                users: Vec::new(),
                current_view: ViewState::LoginScreen,
            },
            // Immediately dispatch our AppStarted message when the GUI boots
            Task::done(Message::AppStarted),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AppStarted => {
                self.status_message = "Connecting to database...".to_string();

                // Spawn a Tokio async task to connect to the DB in the background
                Task::perform(connect_to_db(), |res| {
                    Message::DatabaseConnected(res.map_err(|e| format!("{:#}", e)))
                })
            }

            Message::DatabaseConnected(conn) => match conn {
                Ok(pool) => {
                    let pool_clone = pool.clone();
                    self.pool = Some(pool);
                    self.status_message = "Connected! Loading Users...".to_string();
                    Task::perform(
                        async move { db::operations::get_all_users(&pool_clone).await },
                        |result| match result {
                            Ok(users) => Message::UsersLoaded(users),
                            Err(e) => Message::DatabaseConnected(Err(e.to_string())),
                        },
                    )
                }
                Err(e) => {
                    self.is_loading = false;
                    self.status_message = format!("Database Error: {}", e);
                    Task::none()
                }
            },
            Message::UsersLoaded(users) => {
                self.is_loading = false;
                self.users = users;
                self.status_message = "Users loaded successfully!".to_string();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // A simple centered text display for our bootstrapping phase
        let content = match self.current_view {
            ViewState::LoginScreen => text("Login Screen"),
            ViewState::Loading => text(self.status_message.clone()),
        };
        // let user_elements: Vec<Element<'_, Message> = &self.users.iter().map(|user| text(user.username.clone()).into()).collect();
        // let user_elements: Vec<Element<'_, Message>> = self
        //     .users
        //     .iter()
        //     .map(|user| text(user.username.clone()).into())
        //     .collect();
        // let list = column(user_elements).spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill) // Centers horizontally
            .center_y(Length::Fill) // Centers vertically
            .into()
    }

    pub fn run() -> iced::Result {
        iced::application(Self::new, Self::update, Self::view)
            .title("Anti-Jira")
            .run()
    }
}
