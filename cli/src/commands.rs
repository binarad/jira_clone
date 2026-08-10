use clap::Args;
use db::{connect_to_db, operations::update_issue_status};
use jira_core::issue::{Issue, IssuePriority, IssueStatus, IssueType};

#[derive(Args, Debug)]
pub struct CreateIssueCmd {
    #[arg(short, long)]
    pub project_id: i32,
    #[arg(short, long)]
    pub summary: String,
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(long)]
    pub issue_type: IssueType,
    #[arg(long)]
    pub issue_priority: IssuePriority,
    #[arg(short, long)]
    pub assignee_id: Option<i32>,
    #[arg(short, long)]
    pub reporter_id: i32,
}

#[derive(Args, Debug)]
pub struct ChangeIssueStatusCmd {
    pub issue_id: i32,

    #[arg(long)]
    pub status: Option<IssueStatus>,
    // #[arg(short, long)]
    // pub summary: Option<String>,

    // #[arg(long)]
    // pub priority: Option<IssuePriority>,
}

pub async fn handle_create_issue(cmd: CreateIssueCmd) {
    let pool = connect_to_db().await.unwrap();

    let new_issue = Issue {
        id: 0,
        issue_number: None,
        project_id: cmd.project_id,
        issue_type: cmd.issue_type,
        summary: cmd.summary,
        description: cmd.description,
        status: IssueStatus::default(),
        priority: cmd.issue_priority,
        assignee_id: cmd.assignee_id,
        reporter_id: cmd.reporter_id,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let (id, number) = db::operations::create_issue(&pool, &new_issue)
        .await
        .unwrap();

    println!("Issue #{} successfully created with ID: {}", number, id);
}

pub async fn handle_issue_status_change(cmd: ChangeIssueStatusCmd) {
    let pool = connect_to_db().await.unwrap();

    let Some(new_status) = cmd.status else {
        println!(
            "Error: No status provided. Please use the --status flag (e.g., --status InProgress)"
        );
        return;
    };
    match update_issue_status(&pool, cmd.issue_id, &new_status).await {
        Ok(_) => println!(
            "Successfully updated Issue #{} to {}",
            cmd.issue_id, new_status
        ),
        Err(e) => println!("Failed to update Issue #{}: {}", cmd.issue_id, e),
    }
}
