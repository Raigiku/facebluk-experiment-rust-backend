mod mutations;
mod queries;

use std::sync::Arc;

use crate::Config;

pub struct FileStorageImpl {
    config: Arc<Config>,
}

impl FileStorageImpl {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl integrator::FileStorage for FileStorageImpl {}
