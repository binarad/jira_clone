// use clap::{Parser, Subcommand};
// use comfy_table::{
//     Attribute, Cell, CellAlignment, Color, ColumnConstraint, Table, Width::Fixed,
//     modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL,
// };
// use db::{
//     connect_to_db,
//     operations::{
//         delete_user, get_all_projects, get_all_users, get_project_issues, read_user_projects,
//     },
//     traits::Creatable,
// };
// use std::fmt::Debug;

// use crate::commands::{
//     ChangeIssueStatusCmd, CreateIssueCmd, handle_create_issue, handle_issue_status_change,
// };

// pub mod commands;

// #[derive(Subcommand, Debug)]
// enum Operation {
//     /// Manage Users
//     User {
//         #[command(subcommand)]
//         action: UserAction,
//     },

//     /// Manage Projects
//     Project {
//         #[command(subcommand)]
//         action: ProjectAction,
//     },

//     /// Manage Issues
//     Issue {
//         #[command(subcommand)]
//         action: IssueAction,
//     },
// }

// #[derive(Subcommand, Debug)]
// enum UserAction {
//     /// Prints list of all users
//     List,
//     /// Create User
//     CreateUser {
//         #[command(flatten)]
//         user: User,
//     },

//     /// Delete user by User ID
//     Delete { user_id: i32 },

//     /// Prints user related projects by user ID
//     Projects { user_id: i32 },
// }

// #[derive(Subcommand, Debug)]
// enum ProjectAction {
//     /// Prints all existing projects
//     List,

//     /// Create a new projects
//     Create {
//         #[command(flatten)]
//         project: Project,
//     },

//     /// Shows issues board by the project ID
//     Board { project_id: i32 },
// }

// #[derive(Subcommand, Debug)]
// enum IssueAction {
//     /// Create new issue for the project
//     Create {
//         #[command(flatten)]
//         issue: CreateIssueCmd,
//     },

//     /// Change issue status by the issue ID
//     ChangeStatus {
//         #[command(flatten)]
//         cmd: ChangeIssueStatusCmd,
//     },
// }

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     #[command(subcommand)]
//     operation: Operation,
// }

// #[tokio::main]
// async fn main() {
//     let args = Args::parse();

//     match args.operation {
//         Operation::User { action } => match action {
//             UserAction::List => show_all_users().await,
//             UserAction::CreateUser { user } => show_created_entity(user).await,
//             UserAction::Delete { user_id } => show_deleted_user(user_id).await,
//             UserAction::Projects { user_id } => show_user_projects(user_id).await,
//         },

//         Operation::Project { action } => match action {
//             ProjectAction::List => show_projects().await,
//             ProjectAction::Create { project } => show_created_entity(project).await,
//             ProjectAction::Board { project_id } => board_view_test(project_id).await,
//         },

//         Operation::Issue { action } => match action {
//             IssueAction::Create { issue } => handle_create_issue(issue).await,
//             IssueAction::ChangeStatus { cmd } => handle_issue_status_change(cmd).await,
//         },
//     }
// }

// async fn show_all_users() {
//     let pool = connect_to_db().await.unwrap();
//     let users = get_all_users(&pool).await.unwrap();
//     let mut table = Table::new();
//     table.load_preset(UTF8_FULL);
//     table.apply_modifier(UTF8_ROUND_CORNERS);
//     table.set_header(vec![
//         "User ID",
//         "Username",
//         "Role",
//         "Email",
//         "Password",
//         "Created At",
//     ]);
//     for user in users {
//         table.add_row(vec![
//             user.id.map_or("N/A".to_string(), |id| id.to_string()),
//             user.username,
//             user.role,
//             user.email,
//             user.password_hash,
//             user.created_at.to_string(),
//         ]);
//     }

//     println!("{table}");
// }

// async fn show_user_projects(user_id: i32) {
//     let pool = connect_to_db().await.unwrap();
//     let projects = read_user_projects(&pool, user_id).await.unwrap();
//     let mut table = Table::new();
//     table.load_preset(UTF8_FULL);
//     table.apply_modifier(UTF8_ROUND_CORNERS);
//     table.set_header(vec!["User ID", "Project ID", "Project Name", "Created At"]);

//     for project in projects {
//         table.add_row(vec![
//             user_id.to_string(),
//             project.id.to_string(),
//             project.name,
//             project.created_at.to_string(),
//         ]);
//     }

//     println!("{table}")
// }

// async fn show_created_entity<T: Creatable>(entity: T)
// where
//     T::Output: Debug,
// {
//     let pool = connect_to_db().await.unwrap();

//     match entity.create_in_db(&pool).await {
//         Ok(new_id) => println!(
//             "{} successfully created with ID: {:?}",
//             T::ENTITY_NAME,
//             new_id
//         ),
//         Err(e) => println!(
//             "An error occured while creating new {} : {}",
//             T::ENTITY_NAME,
//             e
//         ),
//     }
// }

// async fn board_view_test(project_id: i32) {
//     let pool = connect_to_db().await.unwrap();
//     let issues = get_project_issues(&pool, project_id).await.unwrap();
//     let mut table = Table::new();
//     table.load_preset(UTF8_FULL);
//     table.apply_modifier(UTF8_ROUND_CORNERS);
//     let header_names = vec![
//         "ID",
//         "#",
//         "Type",
//         "Summary",
//         "Description",
//         "Priority",
//         "Status",
//         "Assignee ID",
//         "Reporter ID",
//         "Created At",
//         "Updated At",
//     ];

//     // Map them into styled Cells
//     let styled_headers = header_names.into_iter().map(|name| {
//         Cell::new(name)
//             .add_attribute(Attribute::Bold)
//             .set_alignment(CellAlignment::Center)
//     });

//     // Set the styled cells as your header
//     table.set_header(styled_headers);
//     if let Some(summary_col) = table.column_mut(2) {
//         summary_col.set_constraint(ColumnConstraint::Absolute(Fixed(40)));
//     }

//     if let Some(desc_col) = table.column_mut(3) {
//         desc_col.set_constraint(ColumnConstraint::Absolute(Fixed(40)));
//     }

//     for issue in issues {
//         let status_cell = match issue.status {
//             IssueStatus::Open => Cell::new("Open").fg(Color::DarkGrey),
//             IssueStatus::InProgress => Cell::new("InProgress").fg(Color::Rgb {
//                 r: 126,
//                 g: 156,
//                 b: 216,
//             }), // Blue
//             IssueStatus::Resolved => Cell::new("Resolved").fg(Color::Rgb {
//                 r: 152,
//                 g: 187,
//                 b: 108,
//             }), // Green
//             IssueStatus::Closed => Cell::new("Closed").fg(Color::Grey),
//         };
//         // Style the Priority
//         let priority_cell = match issue.priority {
//             IssuePriority::Low => Cell::new("Low").fg(Color::DarkGrey),
//             IssuePriority::Medium => Cell::new("Medium").fg(Color::Rgb {
//                 r: 220,
//                 g: 165,
//                 b: 97,
//             }), // Yellowish
//             IssuePriority::High => Cell::new("High").fg(Color::Rgb {
//                 r: 255,
//                 g: 158,
//                 b: 59,
//             }), // Orange
//             IssuePriority::Urgent => Cell::new("Urgent").fg(Color::Rgb {
//                 r: 195,
//                 g: 64,
//                 b: 67,
//             }), // Red
//         };

//         table.add_row(vec![
//             Cell::new(issue.id),
//             Cell::new(issue.issue_number.unwrap()),
//             Cell::new(issue.issue_type.as_str()),
//             Cell::new(issue.summary),
//             Cell::new(issue.description.unwrap_or_else(|| "N/A".to_string())),
//             // Cell::new(issue.priority.as_str()),
//             priority_cell,
//             status_cell, // Already a styled Cell!
//             Cell::new(
//                 issue
//                     .assignee_id
//                     .map_or("N/A".to_string(), |ass_id| ass_id.to_string()),
//             ),
//             Cell::new(issue.reporter_id.to_string()),
//             Cell::new(issue.created_at.format("%Y-%m-%d %H:%M:%S")),
//             Cell::new(issue.updated_at.format("%Y-%m-%d %H:%M:%S")),
//         ]);
//     }
//     println!("{table}")
// }

// async fn show_projects() {
//     let pool = connect_to_db().await.unwrap();
//     let projects = get_all_projects(&pool).await.unwrap();
//     let mut table = Table::new();
//     table.load_preset(UTF8_FULL);
//     table.apply_modifier(UTF8_ROUND_CORNERS);
//     table.set_header(vec![
//         "Owner ID",
//         "Project ID",
//         "Project Name",
//         "Project Key",
//         "Created At",
//     ]);

//     for project in projects {
//         table.add_row(vec![
//             project.owner_id.to_string(),
//             project.id.to_string(),
//             project.name,
//             project.key,
//             project.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
//         ]);
//     }
//     println!("{table}")
// }

// async fn show_deleted_user(user_id: i32) {
//     let pool = connect_to_db().await.unwrap();
//     match delete_user(&pool, user_id).await {
//         Ok(()) => println!("User with ID: {} was successfully deleted ", user_id),
//         Err(_) => println!("An unexpected error happened while deleting an user"),
//     }
// }
//
//

//! # [Ratatui] `Tabs` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

// FOR FUTURE
// use jira_core::{
//     issue::{IssuePriority, IssueStatus},
//     project::Project,
//     user::User,
// };

use db::{
    connect_to_db,
    operations::{
        get_all_users,
        // delete_user, get_all_projects, get_all_users, get_project_issues, read_user_projects,
    },
};

// FOR FUTURE
//
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Tabs};
use ratatui::{Frame, symbols};

pub struct AppState {
    users: Vec<Row<'static>>,
    table_state: TableState,
    selected_tab: usize,
}

// impl AppState {
//     pub fn new(&self) -> Self {
//         {
//             self.users:
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut selection = 0;

    let pool = connect_to_db().await.unwrap();
    let db_users = get_all_users(&pool).await.unwrap();

    let mut app_state = AppState {
        users: Vec::new(),
        table_state: TableState::default(),
        selected_tab: 0,
    };
    app_state.table_state.select_first();
    app_state.table_state.select_first_column();
    println!("{:?}", app_state.table_state.selected_cell());

    for user in db_users {
        app_state.users.push(Row::new([
            user.id.map_or("N/A".to_string(), |id| id.to_string()),
            user.username,
            user.role,
            user.email,
            user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        ]))
    }

    ratatui::run(|terminal| {
        loop {
            // Pass the mutable state reference into the render function
            terminal.draw(|frame| render(frame, selection, &mut app_state))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Tab => selection = (selection + 1) % 3,
                    // TIP: You would add Up/Down arrow handling here to scroll your table_state!
                    KeyCode::Down | KeyCode::Char('j') => app_state.table_state.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => app_state.table_state.select_previous(),
                    KeyCode::Right | KeyCode::Char('l') => {
                        app_state.table_state.select_next_column()
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        app_state.table_state.select_previous_column()
                    }
                    _ => {}
                }
            }
        }
    })
}

/// Render the UI with tabs.
fn render(frame: &mut Frame, selected_tab: usize, state: &mut AppState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = layout.areas(frame.area()); // .areas() is slightly cleaner than .layout()

    let title = Line::from_iter([
        Span::from("Tabs Widget").bold(),
        Span::from(" (Press 'q' to quit, left/right for tabs, up/down for table)"),
    ]);
    frame.render_widget(title.centered(), top);

    // --- WIDGET INSIDE A WIDGET LOGIC ---

    // 1. Draw the outer boundary (the Block)
    let outer_block = Block::default().borders(Borders::ALL);
    let inner_area = outer_block.inner(main); // Calculates the space *inside* the borders
    frame.render_widget(outer_block, main);

    // 2. Split that inner space into a top row for Tabs, and the rest for Content
    let inner_layout = Layout::vertical([
        Constraint::Length(1), // 1 line for tabs
        Constraint::Length(1), // 1 empty spacer line
        Constraint::Fill(1),   // Remaining space for the Table
    ]);
    let [tabs_area, _spacer, content_area] = inner_layout.areas(inner_area);

    // 3. Render the specific widgets into their designated inner areas
    render_tabs(frame, tabs_area, selected_tab);
    render_content(frame, content_area, state);
}

/// Render the tabs.
pub fn render_tabs(frame: &mut Frame, area: Rect, selected_tab: usize) {
    let tabs = Tabs::new(vec!["Users", "Projects", "Issues"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(selected_tab)
        .divider(symbols::DOT)
        .padding(" ", " ");

    frame.render_widget(tabs, area);
}

/// Render the tab content.
pub fn render_content(frame: &mut Frame, area: Rect, state: &mut AppState) {
    match state.selected_tab {
        0 => {
            render_table(frame, area, state);
        }

        1 => {
            let p = Paragraph::new("Projects Content...").centered();
            frame.render_widget(p, area);
        }

        2 => {
            let p = Paragraph::new("Issues Content...").centered();
            frame.render_widget(p, area);
        }

        _ => unreachable!(),
    };
}

pub fn render_table(frame: &mut Frame<'_>, area: Rect, state: &mut AppState) {
    let header = Row::new(["User ID ", "Username", "Role", "Email", "Created At"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let widths = [
        Constraint::Percentage(5),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];
    // let table = Table::new(state.users.clone(), [Constraint::Fill(1); 5])
    let table = Table::new(state.users.clone(), widths)
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_stateful_widget(table, area, &mut state.table_state);
}
