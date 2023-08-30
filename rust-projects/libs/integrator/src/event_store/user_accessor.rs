use async_trait::async_trait;
use domain::modules::{
    event_store::user::{events::Registered, User},
    shared::errors::UnexpectedError,
};

#[async_trait]
pub trait UserQueries: Sync + Send {
    async fn user_q_alias_exists(&self, alias: &str) -> Result<bool, UnexpectedError>;
    async fn user_q_find_by_id(&self, id: &str) -> Result<Option<User>, UnexpectedError>;
}

#[async_trait]
pub trait UserMutations: Sync + Send {
    async fn user_m_register(&self, event: &Registered) -> Result<(), UnexpectedError>;
}
