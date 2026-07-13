#[cfg(test)]
mod tests {
    use sqlx::PgPool;

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
            summary: "Original Title".to_string(),
            description: Some("Original Description".to_string()),
            status: IssueStatus::Open,
            priority: IssuePriority::Low,
            assignee_id: None,
            created_at: chrono::Utc::now(),
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
}

pub mod operations;

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
