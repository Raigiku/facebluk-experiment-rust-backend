use async_trait::async_trait;
use domain::modules::{shared::{errors::UnexpectedError, datetime::DateTime}, user_auth::user::User};

#[async_trait]
pub trait UserQueries: Sync + Send {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, UnexpectedError>;
}

#[async_trait]
pub trait UserMutations: Sync + Send {
    async fn mark_as_registered(
        &self,
        id: &str,
        registered_at: &DateTime,
    ) -> Result<(), UnexpectedError>;
}
