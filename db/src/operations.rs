use crate::DbError;
use jira_core::issue::{Issue, IssuePriority, IssueStatus, IssueType};
use jira_core::project::Project;
use sqlx::PgPool;
use std::str::FromStr;

pub async fn create_project(
    pool: &PgPool,
    name: &str,
    key: &str,
    owner_id: i32,
) -> Result<i32, DbError> {
    let new_project = sqlx::query!(
        "INSERT INTO projects (name, key, created_at, owner_id) VALUES ($1, $2, $3, $4) RETURNING id",
        name,
        key,
        chrono::Utc::now(),
        owner_id,
    )
    .fetch_one(pool)
    .await?;
    // To-Do maybe make a better validation for creating a new project via match?
    Ok(new_project.id)
}

// MAYBE I'll use it later
//
// pub async fn get_all_projects_by_owner(
//     pool: &PgPool,
//     user_id: i32,
// ) -> Result<Vec<Project>, DbError> {
//     let records = sqlx::query!("SELECT * FROM projects WHERE owner_id = $1", user_id)
//         .fetch_all(pool)
//         .await?;
//
//     let mut filtered_projects = Vec::new();
//
//     for record in records {
//         filtered_projects.push(Project {
//             id: record.id,
//             name: record.name,
//             key: record.key,
//             owner_id: record.owner_id,
//             created_at: record.created_at,
//         });
//     }
//     Ok(filtered_projects)
// }

pub async fn get_user_projects(pool: &PgPool, user_id: i32) -> Result<Vec<Project>, DbError> {
    // We use DISTINCT in case user has multiple roles
    let records = sqlx::query!(
        r#"
    SELECT DISTINCT p.id, p.name, p.key, p.owner_id, p.created_at 
    FROM projects p
    LEFT JOIN issues i ON p.id = i.project_id
    WHERE p.owner_id = $1
        OR i.assignee_id = $1
        OR i.reporter_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut projects = Vec::new();
    for record in records {
        projects.push(Project {
            id: record.id,
            name: record.name,
            key: record.key,
            owner_id: record.owner_id,
            created_at: record.created_at,
        });
    }

    Ok(projects)
}

pub async fn delete_project(pool: &PgPool, project_id: i32) -> Result<(), DbError> {
    sqlx::query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_project_issues(pool: &PgPool, project_id: i32) -> Result<Vec<Issue>, DbError> {
    let records = sqlx::query!("SELECT * FROM issues WHERE project_id = $1", project_id)
        .fetch_all(pool)
        .await?;

    let mut issues = Vec::new();
    for record in records {
        let issue = Issue {
            id: record.id,
            project_id: record.project_id,
            issue_number: record.issue_number,
            // Issue Metadata
            issue_type: IssueType::from_str(&record.issue_type)
                .map_err(|_| anyhow::anyhow!("Invalid issue type: {}", record.issue_type))?,
            summary: record.summary,
            description: record.description,
            status: IssueStatus::from_str(&record.status)
                .map_err(|_| anyhow::anyhow!("Invalid issue status: {}", record.status))?,
            priority: IssuePriority::from_str(&record.priority)
                .map_err(|_| anyhow::anyhow!("Invalid issue priority: {}", record.priority))?,

            // Users involved
            assignee_id: record.assignee_id,
            reporter_id: record.reporter_id,

            // Timestamps
            created_at: record.created_at,
            updated_at: record.updated_at,
        };
        issues.push(issue);
    }
    // Todo do I need handle Errors here?
    Ok(issues)
}

pub async fn create_issue(pool: &PgPool, issue: &Issue) -> Result<i32, DbError> {
    let status_str = issue.status.as_str();
    let priority_str = issue.priority.as_str();
    let issue_type_str = issue.issue_type.as_str();
    let result = sqlx::query!(
        "INSERT INTO issues (project_id, issue_number, issue_type, summary, description, status, priority, assignee_id, reporter_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id",
        issue.project_id,
        issue.issue_number,
        issue_type_str,
        issue.summary,
        issue.description.as_deref(),
        status_str,
        priority_str,
        issue.assignee_id,
        issue.reporter_id,
        issue.created_at,
        issue.updated_at,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.id)
}

pub async fn update_issue(pool: &PgPool, issue: &Issue) -> Result<i32, DbError> {
    let status_str = issue.status.as_str();
    let priority_str = issue.priority.as_str();
    let issue_type_str = issue.issue_type.as_str();
    let result = sqlx::query!(
        "UPDATE issues SET project_id = $1, issue_number = $2, issue_type = $3, summary = $4, description = $5, status = $6, priority = $7, assignee_id = $8, reporter_id = $9, created_at = $10, updated_at = $11 WHERE id = $12 RETURNING id",
        issue.project_id,
        issue.issue_number,
        issue_type_str,
        issue.summary,
        issue.description.as_deref(),
        status_str,
        priority_str,
        issue.assignee_id,
        issue.reporter_id,
        issue.created_at,
        issue.updated_at,
        issue.id,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.id)
}
