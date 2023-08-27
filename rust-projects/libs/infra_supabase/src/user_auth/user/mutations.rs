use async_trait::async_trait;
use domain::modules::shared::{datetime::DateTime, errors::UnexpectedError};
use integrator::user_auth::user_accessor::UserMutations;
use serde::Serialize;

use super::UserAccessor;
use crate::errors::map_http_status_error;
use domain::map_unexpected_error;

#[async_trait]
impl UserMutations for UserAccessor {
    async fn mark_as_registered(
        &self,
        id: &str,
        registered_at: &DateTime,
    ) -> Result<(), UnexpectedError> {
        let url = format!("{}/auth/v1/admin/users/{}", self.config.api_url, id);
        let http_res = reqwest::Client::new()
            .put(url)
            .header("apiKey", &self.config.service_role)
            .bearer_auth(&self.config.service_role)
            .json(&UpdateUserBody {
                user_metadata: Some(UpdateUserBodyUserMetadata {
                    registered_at: Some(registered_at.to_rfc3339()),
                }),
            })
            .send()
            .await
            .map_err(|err| map_unexpected_error!(err))?;

        let response_status_code = http_res.status().clone();
        if response_status_code.is_success() {
            Ok(())
        } else {
            let body_str = http_res
                .text()
                .await
                .map_err(|err| map_unexpected_error!(err))?;
            Err(map_http_status_error(&response_status_code, &body_str))
        }
    }
}

#[derive(Serialize)]
struct UpdateUserBody {
    user_metadata: Option<UpdateUserBodyUserMetadata>,
}

#[derive(Serialize)]
struct UpdateUserBodyUserMetadata {
    #[serde(rename = "registeredAt")]
    registered_at: Option<String>,
}
