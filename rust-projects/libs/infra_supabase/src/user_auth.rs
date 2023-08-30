mod user;

use std::sync::Arc;

use crate::Config;

pub struct UserAuthImpl {
    config: Arc<Config>,
}

impl UserAuthImpl {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl integrator::UserAuth for UserAuthImpl {}
