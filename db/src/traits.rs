use crate::{
    DbError,
    operations::{create_issue, create_project, create_user},
};
use jira_core::{issue::Issue, project::Project, user::User}; // TO-DO add creatable for Issue
use sqlx::PgPool;

pub trait Creatable {
    type Output;
    const ENTITY_NAME: &'static str;
    fn create_in_db(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Self::Output, DbError>> + Send;
}

impl Creatable for User {
    type Output = i32;

    const ENTITY_NAME: &'static str = "User";
    async fn create_in_db(&self, pool: &PgPool) -> Result<Self::Output, DbError> {
        create_user(pool, self).await
    }
}

impl Creatable for Project {
    type Output = i32;
    const ENTITY_NAME: &'static str = "Project";
    async fn create_in_db(&self, pool: &PgPool) -> Result<Self::Output, DbError> {
        create_project(pool, &self.name, &self.key, self.owner_id).await
    }
}

impl Creatable for Issue {
    type Output = (i32, i32);
    const ENTITY_NAME: &'static str = "Issue";
    async fn create_in_db(&self, pool: &PgPool) -> Result<Self::Output, DbError> {
        create_issue(pool, self).await
    }
}
