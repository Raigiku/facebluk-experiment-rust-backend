use async_trait::async_trait;
use domain::modules::shared::errors::UnexpectedError;

#[async_trait]
pub trait FileQueries: Sync + Send {}

#[async_trait]
pub trait FileMutations: Sync + Send {
    async fn m_upload_file(&self, file_path: &str, bytes: Vec<u8>)
        -> Result<String, UnexpectedError>;
}
