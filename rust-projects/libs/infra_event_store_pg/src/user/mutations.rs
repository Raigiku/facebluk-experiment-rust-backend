use async_trait::async_trait;
use domain::{modules::{event_store::user::events::Registered, shared::errors::UnexpectedError}, map_unexpected_error};
use integrator::event_store::user_accessor::UserMutations;

use super::UserAccessor;

#[async_trait]
impl UserMutations for UserAccessor {
    async fn register(&self, event: &Registered) -> Result<(), UnexpectedError> {
        sqlx::query(
            "
                INSERT INTO user (
                    id,
                    version,
                    created_at,
                    alias,
                    name,
                    profile_picture_url
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
        )
        .bind(&event.data.aggregate_id)
        .bind(&event.data.aggregate_version)
        .bind(&event.data.created_at.to_rfc3339())
        .bind(&event.payload.alias)
        .bind(&event.payload.name)
        .bind(&event.payload.profile_picture_url)
        .execute(self.pool.as_ref())
        .await
        .map_err(|err| map_unexpected_error!(err))?;

        Ok(())
    }
}
