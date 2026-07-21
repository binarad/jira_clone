pub struct User {
    id: i32,
    username: String,
    role: String,          // Todo create UserRole struct
    email: String,         // Todo email validation later?
    password_hash: String, // ?
    created_at: chrono::DateTime<chrono::Utc>,
}
