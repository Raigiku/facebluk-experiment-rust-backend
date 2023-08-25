use std::sync::Arc;
pub mod mutations;
pub mod queries;

use sqlx::PgPool;

pub struct UserAccessor {
    pool: Arc<PgPool>,
}

impl UserAccessor {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pool: pg_pool }
    }
}
