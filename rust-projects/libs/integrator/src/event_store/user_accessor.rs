use async_trait::async_trait;
use domain::modules::event_store::user::User;

#[async_trait]
pub trait UserQueries: Sync + Send {
    async fn alias_exists(&self, alias: &str) -> bool;
    async fn find_by_id(&self, id: &str) -> Option<User>;
}

#[async_trait]
pub trait UserMutations: Sync + Send {}
