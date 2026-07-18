use crate::DbError;
use jira_core::issue::{Issue, IssuePriority, IssueStatus};
use sqlx::PgPool;
use std::str::FromStr;

pub async fn create_project(pool: &PgPool, name: &str, key: &str) -> Result<i32, DbError> {
    let new_project = sqlx::query!(
        "INSERT INTO projects (name, key, created_at) VALUES ($1, $2, $3) RETURNING id",
        name,
        key,
        chrono::Utc::now()
    )
    .fetch_one(pool)
    .await?;
    // To-Do maybe make a better validation for creating a new project via match?
    Ok(new_project.id)
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
            summary: record.summary,
            description: record.description,
            status: IssueStatus::from_str(&record.status)
                .map_err(|_| anyhow::anyhow!("Invalid issue status: {}", record.status))?,
            priority: IssuePriority::from_str(&record.priority)
                .map_err(|_| anyhow::anyhow!("Invalid issue priority: {}", record.priority))?,
            assignee_id: record.assignee_id,
            created_at: record.created_at,
        };
        issues.push(issue);
    }
    // Todo do I need handle Errors here?
    Ok(issues)
}

pub async fn create_issue(pool: &PgPool, issue: &Issue) -> Result<i32, DbError> {
    let status_str = issue.status.as_str();
    let priority_str = issue.priority.as_str();
    let result = sqlx::query!(
        "INSERT INTO issues (project_id, issue_number, summary, description, status, priority, assignee_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
        issue.project_id,
        issue.issue_number,
        issue.summary,
        issue.description.as_deref(),
        status_str,
        // issue.status.as_str(),
        // issue.priority.as_str(),
        priority_str,
        issue.assignee_id,
        issue.created_at,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.id)
}

pub async fn update_issue(pool: &PgPool, issue: &Issue) -> Result<i32, DbError> {
    let status_str = issue.status.as_str();
    let priority_str = issue.priority.as_str();
    let result = sqlx::query!(
        "UPDATE issues SET project_id = $1, issue_number = $2, summary = $3, description = $4, status = $5, priority = $6, assignee_id = $7, created_at = $8 WHERE id = $9 RETURNING id",
        issue.project_id,
        issue.issue_number,
        issue.summary,
        issue.description.as_deref(),
        status_str,
        priority_str,
        issue.assignee_id,
        issue.created_at,
        issue.id,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.id)
}
