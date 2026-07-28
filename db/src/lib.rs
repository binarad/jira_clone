pub mod operations;
pub mod traits;

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::operations::{create_project, read_user_projects};

    #[tokio::test]
    async fn test_fetch_issues() {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();

        let issues = super::operations::get_project_issues(&pool, 1)
            .await
            .unwrap();
        println!("Got Issues: {:?}", issues);
    }

    #[tokio::test]
    async fn test_update_issue() {
        use crate::operations::*;
        use jira_core::issue::*;
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();

        // 1. Create a dummy project for the Foreign Key constraint
        let timestamp = chrono::Utc::now().timestamp_subsec_nanos();
        let project_key = format!("UPD{}", timestamp % 10000);

        let project_record = sqlx::query!(
            "INSERT INTO projects (key, name) VALUES ($1, $2) RETURNING id",
            project_key,
            "Update Test Project"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let new_project_id = project_record.id;

        // 2. Build the initial dummy issue
        let mut test_issue = Issue {
            id: 0, // Ignored by create_issue
            project_id: new_project_id,
            issue_number: 1,
            issue_type: IssueType::Task,
            summary: "Original Title".to_string(),
            description: Some("Original Description".to_string()),
            status: IssueStatus::Open,
            priority: IssuePriority::Low,
            assignee_id: None,
            reporter_id: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 3. Insert it into the database
        let inserted_issue_id = create_issue(&pool, &test_issue).await.unwrap();

        // 4. Modify our Rust struct to simulate a user editing the ticket in the UI
        test_issue.id = inserted_issue_id; // CRUCIAL: Set the real ID so the UPDATE knows which row to target!
        test_issue.summary = "Updated Title!".to_string();
        test_issue.status = IssueStatus::InProgress;
        test_issue.priority = IssuePriority::Urgent;

        // 5. Call the update_issue function
        let updated_id = update_issue(&pool, &test_issue).await.unwrap();

        // 6. Assertions! Let's double check the DB actually saved the changes
        assert_eq!(inserted_issue_id, updated_id);

        let verify_record = sqlx::query!(
            "SELECT summary, status, priority FROM issues WHERE id = $1",
            updated_id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(verify_record.summary, "Updated Title!");
        assert_eq!(verify_record.status, "InProgress");
        assert_eq!(verify_record.priority, "Urgent");

        println!(
            "Successfully created and updated test issue (ID: {})!",
            updated_id
        );
    }
    // #[tokio::test]
    // async fn test_create_issue() {
    //     use jira_core::issue::Issue;

    //     dotenvy::dotenv().ok();
    //     let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    //     let pool = PgPool::connect(&db_url).await.unwrap();

    //     // 1. Create a dummy project first to satisfy the FOREIGN KEY constraint.
    //     // We use the current timestamp in the key to ensure the `UNIQUE (key)` constraint
    //     // doesn't fail if we run `cargo test` multiple times!
    //     let timestamp = chrono::Utc::now().timestamp_subsec_nanos();
    //     let project_key = format!("TST{}", timestamp % 10000);

    //     let project_record = sqlx::query!(
    //         "INSERT INTO projects (key, name) VALUES ($1, $2) RETURNING id",
    //         project_key,
    //         "Test Project for Issue Creation"
    //     )
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();

    //     let new_project_id = project_record.id;

    //     let issue = Issue {
    //         id: 1,
    //         project_id: 1,
    //         issue_number: 1,
    //         summary: "Test Issue".to_string(),
    //         description: Some("This is a test issue".to_string()),
    //         status: jira_core::issue::IssueStatus::Closed,
    //         priority: jira_core::issue::IssuePriority::High,
    //         assignee_id: None,
    //         created_at: chrono::Utc::now(),
    //     };

    //     let result = super::operations::create_issue(&pool, &issue)
    //         .await
    //         .unwrap();
    //     println!(
    //         "Successfully created test project (ID: {}) and test issue (ID: {})!",
    //         new_project_id, result
    //     );
    //     assert!(result > 0);
    // }

    #[tokio::test]
    async fn test_project_crud_workflow() {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();

        let user_record = sqlx::query!(
            "INSERT INTO users (username, role, email, password_hash) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id",
            "project_tester",
            "admin",
            "tester@test.com",
            "hash"
        ).fetch_optional(&pool).await.unwrap();

        let user_id = match user_record {
            Some(record) => record.id,
            None => {
                sqlx::query!("SELECT id FROM users WHERE username = 'project_tester'")
                    .fetch_one(&pool)
                    .await
                    .unwrap()
                    .id
            }
        };

        let timestamp = chrono::Utc::now().timestamp_subsec_nanos();
        let project_key = format!("T{timestamp}", timestamp = timestamp % 100000);
        let project_name = format!("Test Project {}", timestamp);

        let project_id = create_project(&pool, &project_name, &project_key, user_id)
            .await
            .expect("Failed to create project");

        assert!(project_id > 0, "Project id should be valid");

        let user_project = read_user_projects(&pool, user_id)
            .await
            .expect("Failed to fetch user projects");

        assert!(
            user_project.iter().any(|p| p.id == project_id),
            "Newly created project should be appear in user's project list"
        );
    }

    #[tokio::test]
    async fn create_user_test() {
        use crate::operations::create_user;
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();

        let new_user = jira_core::user::User {
            username: "John Doe".to_string(),
            role: "Admin".to_string(),
            email: "johndoepool@gmail.com".to_string(),
            password_hash: "29299ajajasdasdaaaaaaakakak".to_string(),
            created_at: chrono::Utc::now(),
            ..Default::default()
        };

        let result = create_user(&pool, &new_user).await.unwrap();
        println!("Successfully created user with id {}", result);
        assert!(result > 0);
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum DbError {
    // Automatically converts sqlx errors into this variant when using the `?` operator
    #[error("Database query failed: {0}")]
    Query(#[from] sqlx::Error),

    #[error("The requested project (ID: {0}) was not found")]
    ProjectNotFound(i32),

    #[error("Database connection pool timed out")]
    Timeout,

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}

use anyhow::{Context, Result};
use sqlx::PgPool;
pub async fn connect_to_db() -> Result<PgPool> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")
        .context("Missing 'DATABASE_URL' in the .env file. Please check your configuration")?;
    let pool = PgPool::connect(&db_url)
        .await
        .context("Failed to establish a connection to Postgres. Is docker running?")?;
    Ok(pool)
}
