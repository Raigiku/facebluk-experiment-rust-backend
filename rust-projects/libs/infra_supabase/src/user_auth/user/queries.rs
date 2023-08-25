use domain::modules::{shared::errors::UnexpectedError, user_auth::user::User};
use reqwest::StatusCode;
use serde::Deserialize;

use async_trait::async_trait;
use integrator::user_auth::user_accessor::UserQueries;

use crate::errors::{map_http_status_error, map_reqwest_error};

use super::UserAccessor;

#[async_trait]
impl UserQueries for UserAccessor {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, UnexpectedError> {
        let url = format!("{}/user-auth", self.config.api_url);
        let query_params = &[("function", "find_user_by_id"), ("user_id", id)];
        let http_res = reqwest::Client::new()
            .get(url)
            .query(query_params)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let response_status_code = http_res.status().clone();
        if response_status_code.is_success() {
            let res_dto = http_res
                .json::<UserDTO>()
                .await
                .map_err(map_reqwest_error)?;
            Ok(Some(User::from_existing(res_dto.id, res_dto.registered_at.map(|x| x.try_into().unwrap()))))
        } else if response_status_code == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            let body_str = http_res.text().await.map_err(map_reqwest_error)?;
            Err(map_http_status_error(&response_status_code, &body_str))
        }
    }
}

#[derive(Deserialize)]
struct UserDTO {
    id: String,
    registered_at: Option<String>,
}
