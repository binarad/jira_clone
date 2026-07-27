use clap::Parser;
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use db::{
    connect_to_db,
    operations::{create_user, get_all_users, read_user_projects},
};
use jira_core::user::User;

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
        Operation::CreateUser { user } => show_created_user(user).await,
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

async fn show_created_user(user: User) {
    let pool = connect_to_db().await.unwrap();
    let new_user = match create_user(&pool, &user).await {
        Ok(new_user_id) => new_user_id,
        Err(e) => {
            println!("An Error Occured: {}", e);
            return;
        }
    };

    println!("User successfully created with ID: {}", new_user);
}
