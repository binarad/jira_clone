use anyhow::Result;
use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct Issue {
    pub id: i32,
    pub project_id: i32,
    pub issue_number: Option<i32>,

    // Issue metadata
    pub issue_type: IssueType,
    pub summary: String,
    pub description: Option<String>,
    pub status: IssueStatus,
    pub priority: IssuePriority,

    // Users involved
    pub assignee_id: Option<i32>,
    pub reporter_id: i32,

    // Timestamps
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum IssueStatus {
    #[default]
    Open,
    InProgress,
    Resolved,
    Closed,
}
impl Display for IssueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl IssueStatus {
    pub fn as_str(&self) -> &str {
        match self {
            IssueStatus::Open => "Open",
            IssueStatus::InProgress => "InProgress",
            IssueStatus::Resolved => "Resolved",
            IssueStatus::Closed => "Closed",
        }
    }
}
impl FromStr for IssueStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Open" => Ok(IssueStatus::Open),
            "InProgress" => Ok(IssueStatus::InProgress),
            "Resolved" => Ok(IssueStatus::Resolved),
            "Closed" => Ok(IssueStatus::Closed),
            _ => Err(anyhow::anyhow!("Invalid issue status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssuePriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl IssuePriority {
    pub fn as_str(&self) -> &str {
        match self {
            IssuePriority::Low => "Low",
            IssuePriority::Medium => "Medium",
            IssuePriority::High => "High",
            IssuePriority::Urgent => "Urgent",
        }
    }
}

impl FromStr for IssuePriority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(IssuePriority::Low),
            "Medium" => Ok(IssuePriority::Medium),
            "High" => Ok(IssuePriority::High),
            "Urgent" => Ok(IssuePriority::Urgent),
            _ => Err(anyhow::anyhow!("Invalid issue priority: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueType {
    Bug,
    Task,
    Feature,
}

impl IssueType {
    pub fn as_str(&self) -> &str {
        // TODO MAKE A PROPER VALIDATION
        match self {
            IssueType::Bug => "Bug",
            IssueType::Task => "Task",
            IssueType::Feature => "Feature",
        }
    }
}

impl FromStr for IssueType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bug" => Ok(IssueType::Bug),
            "Task" => Ok(IssueType::Task),
            "Feature" => Ok(IssueType::Feature),
            _ => Err(anyhow::anyhow!("Invalid issue type: {s}")),
        }
    }
}
