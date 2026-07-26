use jira_core::{project::Project, user::User};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub enum Message {
    AppStarted,
    DatabaseConnected(Result<PgPool, String>),
    UsersLoaded(Vec<User>),
    // Project fields
    // ProjectCreated,
    // ProjectSelected(i32),
    // ProjectsLoaded(Vec<Project>),

    // Issue Fields
    // IssueCreated,
    // IssuesLoaded,
}
