use clap::Parser;
use comfy_table::{
    Attribute, Cell, CellAlignment, Color, ColumnConstraint, Table, Width::Fixed,
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL,
};
use db::{
    connect_to_db,
    operations::{
        delete_user, get_all_projects, get_all_users, get_project_issues, read_user_projects,
    },
    traits::Creatable,
};
use jira_core::{issue::IssuePriority, issue::IssueStatus, project::Project, user::User};

#[derive(Parser, Debug)]
enum Operation {
    /// Prints list of all users
    Users,

    /// Create user
    CreateUser {
        #[command(flatten)]
        user: User,
    },

    /// Delete user by User_ID
    DeleteUser {
        user_id: i32,
    },

    /// Create a new project
    CreateProject {
        #[command(flatten)]
        project: Project,
    },

    // Prints all existing projects
    Projects,

    /// Prints user related projects
    UserProject {
        user_id: i32,
    },

    /// Shows issues board by the project
    Board {
        project_id: i32,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    operation: Operation,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.operation {
        Operation::Users => show_all_users().await,
        Operation::UserProject { user_id } => show_user_projects(user_id).await,
        // Operation::CreateUser { user } => show_created_user(user).await,
        Operation::CreateUser { user } => show_created_entity(user).await,
        Operation::DeleteUser { user_id } => show_deleted_user(user_id).await,
        Operation::CreateProject { project } => show_created_entity(project).await,
        Operation::Projects => show_projects().await,
        Operation::Board { project_id } => board_view_test(project_id).await,
    };
}

async fn show_all_users() {
    let pool = connect_to_db().await.unwrap();
    let users = get_all_users(&pool).await.unwrap();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec![
        "User ID",
        "Username",
        "Role",
        "Email",
        "Password",
        "Created At",
    ]);
    for user in users {
        table.add_row(vec![
            user.id.map_or("N/A".to_string(), |id| id.to_string()),
            user.username,
            user.role,
            user.email,
            user.password_hash,
            user.created_at.to_string(),
        ]);
    }

    println!("{table}");
}

async fn show_user_projects(user_id: i32) {
    let pool = connect_to_db().await.unwrap();
    let projects = read_user_projects(&pool, user_id).await.unwrap();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec!["User ID", "Project ID", "Project Name", "Created At"]);

    for project in projects {
        table.add_row(vec![
            user_id.to_string(),
            project.id.to_string(),
            project.name,
            project.created_at.to_string(),
        ]);
        // println!("{:?}", project);
    }

    println!("{table}")
}

async fn show_created_entity<T: Creatable>(entity: T) {
    let pool = connect_to_db().await.unwrap();

    match entity.create_in_db(&pool).await {
        Ok(new_id) => println!(
            "{} successfully created with ID: {}",
            T::ENTITY_NAME,
            new_id
        ),
        Err(e) => println!(
            "An error occured while creating new {} : {}",
            T::ENTITY_NAME,
            e
        ),
    }
}

async fn board_view_test(project_id: i32) {
    let pool = connect_to_db().await.unwrap();
    let issues = get_project_issues(&pool, project_id).await.unwrap();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    let header_names = vec![
        "#",
        "Type",
        "Summary",
        "Description",
        "Priority",
        "Status",
        "Assignee ID",
        "Reporter ID",
        "Created At",
        "Updated At",
    ];

    // 2. Map them into styled Cells
    let styled_headers = header_names.into_iter().map(|name| {
        Cell::new(name)
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center)
    });

    // 3. Set the styled cells as your header
    table.set_header(styled_headers);
    if let Some(summary_col) = table.column_mut(2) {
        summary_col.set_constraint(ColumnConstraint::Absolute(Fixed(40)));
    }

    if let Some(desc_col) = table.column_mut(3) {
        desc_col.set_constraint(ColumnConstraint::Absolute(Fixed(40)));
    }

    for issue in issues {
        let status_cell = match issue.status {
            IssueStatus::Open => Cell::new("Open").fg(Color::DarkGrey),
            IssueStatus::InProgress => Cell::new("InProgress").fg(Color::Rgb {
                r: 126,
                g: 156,
                b: 216,
            }), // Blue
            IssueStatus::Resolved => Cell::new("Resolved").fg(Color::Rgb {
                r: 152,
                g: 187,
                b: 108,
            }), // Green
            IssueStatus::Closed => Cell::new("Closed").fg(Color::Grey),
        };
        // 2. Style the Priority
        let priority_cell = match issue.priority {
            IssuePriority::Low => Cell::new("Low").fg(Color::DarkGrey),
            IssuePriority::Medium => Cell::new("Medium").fg(Color::Rgb {
                r: 220,
                g: 165,
                b: 97,
            }), // Yellowish
            IssuePriority::High => Cell::new("High").fg(Color::Rgb {
                r: 255,
                g: 158,
                b: 59,
            }), // Orange
            IssuePriority::Urgent => Cell::new("Urgent").fg(Color::Rgb {
                r: 195,
                g: 64,
                b: 67,
            }), // Red
        };

        table.add_row(vec![
            Cell::new(issue.issue_number),
            Cell::new(issue.issue_type.as_str()),
            Cell::new(issue.summary),
            Cell::new(issue.description.unwrap_or_else(|| "N/A".to_string())),
            // Cell::new(issue.priority.as_str()),
            priority_cell,
            status_cell, // Already a styled Cell!
            Cell::new(
                issue
                    .assignee_id
                    .map_or("N/A".to_string(), |ass_id| ass_id.to_string()),
            ),
            Cell::new(issue.reporter_id.to_string()),
            Cell::new(issue.created_at.format("%Y-%m-%d %H:%M:%S")),
            Cell::new(issue.updated_at.format("%Y-%m-%d %H:%M:%S")),
        ]);
    }
    println!("{table}")
}

async fn show_projects() {
    let pool = connect_to_db().await.unwrap();
    let projects = get_all_projects(&pool).await.unwrap();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec![
        "Owner ID",
        "Project ID",
        "Project Name",
        "Project Key",
        "Created At",
    ]);

    for project in projects {
        table.add_row(vec![
            project.owner_id.to_string(),
            project.id.to_string(),
            project.name,
            project.key,
            project.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        ]);
    }
    println!("{table}")
}

async fn show_deleted_user(user_id: i32) {
    let pool = connect_to_db().await.unwrap();
    match delete_user(&pool, user_id).await {
        Ok(()) => println!("User with ID: {} was successfully deleted ", user_id),
        Err(_) => println!("An unexpected error happened while deleting an user"),
    }
}
