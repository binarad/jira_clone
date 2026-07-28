use clap::Parser;
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use db::{
    connect_to_db,
    operations::{get_all_users, read_user_projects},
    traits::Creatable,
};
use jira_core::{project::Project, user::User};

#[derive(Parser, Debug)]
enum Operation {
    /// Prints list of all users
    Users,

    /// Create User
    CreateUser {
        #[command(flatten)]
        user: User,
    },

    /// Prints user related projects
    UserProject { user_id: i32 },

    /// Create a new project
    CreateProject {
        #[command(flatten)]
        project: Project,
    },
}

/// Shows kanban board by the project
// Board { project_id: i32 },
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
        Operation::CreateProject { project } => show_created_entity(project).await,
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
// async fn show_created_user(user: User) {
//     let pool = connect_to_db().await.unwrap();
//     let new_user = match create_user(&pool, &user).await {
//         Ok(new_user_id) => new_user_id,
//         Err(e) => {
//             println!("An Error Occured: {}", e);
//             return;
//         }
//     };
//
//     println!("User successfully created with ID: {}", new_user);
// }
//
// async fn show_created_project(project: Project) {
//     let pool = connect_to_db().await.unwrap();
//     let new_project =
//         match create_project(&pool, &project.name, &project.key, project.owner_id).await {
//             Ok(project_id) => project_id,
//             Err(e) => {
//                 println!("An error occured while creating a new project: {}", e);
//                 return;
//             }
//         };
//
//     println!("Project successfully created with ID: {}", new_project);
// }
