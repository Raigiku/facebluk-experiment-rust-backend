use async_trait::async_trait;
use integrator::file_storage::file_accessor::FileQueries;

use super::FileStorageImpl;

#[async_trait]
impl FileQueries for FileStorageImpl {}
