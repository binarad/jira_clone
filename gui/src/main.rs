use anyhow::{Context, Result};
use iced::widget::{container, text};
use iced::{Element, Length, Task};
use sqlx::PgPool;

pub fn main() -> iced::Result {
    iced::application(JiraApp::new, JiraApp::update, JiraApp::view)
        .title("Anti-Jira")
        .run()
}

// Our core application state
struct JiraApp {
    is_loading: bool,
    status_message: String,
    pool: Option<PgPool>,
}

// The messages that drive our application's state machine
#[derive(Debug, Clone)]
pub enum Message {
    AppStarted,
    DatabaseConnected(Result<PgPool, String>),
}

impl JiraApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                is_loading: true,
                status_message: "Initializing...".to_string(),
                pool: None,
            },
            // Immediately dispatch our AppStarted message when the GUI boots
            Task::done(Message::AppStarted),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AppStarted => {
                self.status_message = "Connecting to database...".to_string();

                // Spawn a Tokio async task to connect to the DB in the background
                Task::perform(connect_to_db(), |res| {
                    Message::DatabaseConnected(res.map_err(|e| format!("{:#}", e)))
                })
            }
            Message::DatabaseConnected(Ok(pool)) => {
                self.is_loading = false;
                self.status_message = "Connected! Welcome to Anti-Jira.".to_string();
                self.pool = Some(pool);
                Task::none()
            }
            Message::DatabaseConnected(Err(e)) => {
                self.is_loading = false;
                self.status_message = format!("Database Error: {}", e);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // A simple centered text display for our bootstrapping phase
        let content = text(&self.status_message).size(24);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill) // Centers horizontally
            .center_y(Length::Fill) // Centers vertically
            .into()
    }
}

// A mock async function to simulate database connection time
// In the next step, we will replace this with your actual sqlx PgPool connection!
async fn connect_to_db() -> Result<PgPool> {
    // tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")
        .context("Missing 'DATABASE_URL' in the .env file. Please check your configuration")?;
    let pool = PgPool::connect(&db_url)
        .await
        .context("Failed to establish a connection to Postgres. Is docker running?")?;
    // Simulate a success
    Ok(pool)
}
