use std::sync::Arc;

use domain::config::SharedConfig;
use integrator::{EventStore, FileStorage, MsgBroker, UserAuth};

use super::config::Config;

pub struct AppState {
    pub api_config: Config,
    pub shared_config: SharedConfig,
    pub file_storage: Arc<dyn FileStorage>,
    pub user_auth: Arc<dyn UserAuth>,
    pub event_store: Arc<dyn EventStore>,
    pub msg_broker: Arc<dyn MsgBroker>,
}

impl AppState {
    pub async fn new(
        api_config: Config,
        file_storage: Arc<dyn FileStorage>,
        user_auth: Arc<dyn UserAuth>,
        event_store: Arc<dyn EventStore>,
        msg_broker: Arc<dyn MsgBroker>,
    ) -> Self {
        Self {
            api_config,
            shared_config: SharedConfig::new(),
            file_storage,
            user_auth,
            event_store,
            msg_broker,
        }
    }
}
