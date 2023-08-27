use domain::modules::{
    shared::{datetime::DateTime, errors::UnexpectedError},
    user_auth::user::User,
};
use reqwest::StatusCode;
use serde::Deserialize;

use async_trait::async_trait;
use integrator::user_auth::user_accessor::UserQueries;

use super::UserAccessor;
use crate::errors::map_http_status_error;
use domain::map_unexpected_error;

#[async_trait]
impl UserQueries for UserAccessor {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, UnexpectedError> {
        let url = format!("{}/auth/v1/admin/users/{}", self.config.api_url, id);
        let http_res = reqwest::Client::new()
            .get(url)
            .header("apiKey", &self.config.service_role)
            .bearer_auth(&self.config.service_role)
            .send()
            .await
            .map_err(|err| map_unexpected_error!(err))?;

        let response_status_code = http_res.status().clone();
        if response_status_code.is_success() {
            let res_dto = http_res
                .json::<UserDTO>()
                .await
                .map_err(|err| map_unexpected_error!(err))?;

            let registered_at: Option<DateTime> = res_dto
                .user_metadata
                .registered_at
                .map(|x| x.try_into())
                .transpose()?;

            Ok(Some(User::from_existing(res_dto.id, registered_at)))
        } else if response_status_code == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            let body_str = http_res
                .text()
                .await
                .map_err(|err| map_unexpected_error!(err))?;
            Err(map_http_status_error(&response_status_code, &body_str))
        }
    }
}

#[derive(Deserialize)]
struct UserDTO {
    id: String,
    user_metadata: UserDTOMetadata,
}

#[derive(Deserialize)]
struct UserDTOMetadata {
    #[serde(alias = "registeredAt")]
    registered_at: Option<String>,
}
