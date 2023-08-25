mod config;
mod user;

use std::sync::Arc;

use integrator::event_store::user_accessor::{UserMutations, UserQueries};
pub use user::UserAccessor;
pub use config::Config;
use sqlx::postgres::{PgPool,PgConnectOptions, PgPoolOptions};

pub async fn new_pool(config: Config) -> Result<PgPool, sqlx::Error> {
    let options = PgConnectOptions::new()
        .host(&config.host)
        .database(&config.db_name)
        .username(&config.username)
        .password(&config.password)
        .port(config.port);

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

pub struct EventStoreImpl {
    user_queries: Arc<dyn UserQueries>,
    user_mutations: Arc<dyn UserMutations>,
}

impl EventStoreImpl {
    pub fn new(user_queries: Arc<dyn UserQueries>, user_mutations: Arc<dyn UserMutations>) -> Self {
        Self {
            user_queries,
            user_mutations,
        }
    }

}

impl integrator::EventStore for EventStoreImpl {
    fn user_queries(&self) -> &Arc<dyn UserQueries> {
        &self.user_queries
    }

    fn user_mutations(&self) -> &Arc<dyn UserMutations> {
        &self.user_mutations
    }
}