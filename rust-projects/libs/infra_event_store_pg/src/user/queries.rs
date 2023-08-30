use async_trait::async_trait;
use domain::modules::{
    event_store::{aggregate::Aggregate, user::User},
    shared::errors::UnexpectedError,
};
use integrator::event_store::user_accessor::UserQueries;
use std::collections::HashMap;

use domain::map_unexpected_error;

use crate::EventStoreImpl;

#[async_trait]
impl UserQueries for EventStoreImpl {
    async fn user_q_alias_exists(&self, alias: &str) -> Result<bool, UnexpectedError> {
        Ok(sqlx::query(
            "
            SELECT 1
            FROM \"user\" u
            WHERE u.alias = $1
            ",
        )
        .bind(alias)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|err| map_unexpected_error!(err))?
        .is_some())
    }

    async fn user_q_find_by_id(&self, id: &str) -> Result<Option<User>, UnexpectedError> {
        let sql_str = format!("{} {}", QUERY_SQL, "WHERE u.id = $1");
        let rows = sqlx::query_as::<_, QueryRow>(&sql_str)
            .bind(id)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|err| map_unexpected_error!(err))?;
        let mut rows = map_query_rows(rows)?;
        if rows.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(rows.remove(0)))
        }
    }
}

const QUERY_SQL: &str = "
    SELECT
        u.id as u_id
        ,u.version as u_version,
        ,u.created_at as u_created_at,
        ,u.\"name\" as u_name,
        ,u.alias as u_alias,
        ,u.profile_picture_url as u_profile_picture_url
    FROM \"user\" u
";

#[derive(sqlx::FromRow)]
struct QueryRow {
    u_id: String,
    u_version: i64,
    u_created_at: String,
    u_name: String,
    u_alias: String,
    u_profile_picture_url: Option<String>,
}

fn map_query_rows(rows: Vec<QueryRow>) -> Result<Vec<User>, UnexpectedError> {
    let mut map = HashMap::<String, User>::new();
    for row in rows.into_iter() {
        if !map.contains_key(&row.u_id) {
            map.insert(
                row.u_id.clone(),
                User::from_existing(
                    Aggregate::from_existing(row.u_id, row.u_version, row.u_created_at.try_into()?),
                    row.u_name,
                    row.u_alias,
                    row.u_profile_picture_url,
                ),
            );
        }
    }
    Ok(map.into_values().collect())
}
