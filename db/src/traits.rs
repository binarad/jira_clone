use crate::{
    DbError,
    operations::{create_project, create_user},
};
use jira_core::{project::Project, user::User}; // TO-DO add creatable for Issue
use sqlx::PgPool;

pub trait Creatable {
    const ENTITY_NAME: &'static str;
    fn create_in_db(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<i32, DbError>> + Send;
}

impl Creatable for User {
    const ENTITY_NAME: &'static str = "User";
    async fn create_in_db(&self, pool: &PgPool) -> Result<i32, DbError> {
        create_user(pool, self).await
    }
}

impl Creatable for Project {
    const ENTITY_NAME: &'static str = "Project";
    async fn create_in_db(&self, pool: &PgPool) -> Result<i32, DbError> {
        create_project(pool, &self.name, &self.key, self.owner_id).await
    }
}
