use async_trait::async_trait;

#[async_trait]
pub trait UserQueries: Sync + Send {}

#[async_trait]
pub trait UserMutations: Sync + Send {
    async fn upload_image(&self, user_id: &str, file_name: &str, bytes: Vec<u8>) -> String;
}
