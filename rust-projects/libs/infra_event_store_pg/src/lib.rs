mod config;
mod user;

use std::sync::Arc;

pub use config::Config;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

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
    pool: Arc<sqlx::PgPool>,
}

impl EventStoreImpl {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

impl integrator::EventStore for EventStoreImpl {}
