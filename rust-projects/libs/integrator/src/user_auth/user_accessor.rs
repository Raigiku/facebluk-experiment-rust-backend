use async_trait::async_trait;
use domain::modules::{user_auth::user::User, shared::errors::UnexpectedError};

#[async_trait]
pub trait UserQueries: Sync + Send {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, UnexpectedError>;
}

#[async_trait]
pub trait UserMutations: Sync + Send {}
