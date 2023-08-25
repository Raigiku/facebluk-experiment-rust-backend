use std::sync::Arc;
pub mod mutations;
pub mod queries;

use crate::Config;


pub struct UserAccessor {
    config: Arc<Config>,
}

impl UserAccessor {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
