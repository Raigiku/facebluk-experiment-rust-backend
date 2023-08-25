use async_trait::async_trait;
use domain::modules::shared::errors::UnexpectedError;
use integrator::file_storage::user_accessor::UserMutations;

use crate::errors::map_reqwest_error;

use super::UserAccessor;

#[async_trait]
impl UserMutations for UserAccessor {
    async fn upload_image(
        &self,
        user_id: &str,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> Result<String, UnexpectedError> {
        let file_path = format!("users/{}/{}", user_id, file_name);
        let url = format!(
            "{}/storage/v1/object/images/{}",
            self.config.api_url, file_path
        );
        let upload_img_form =
            reqwest::multipart::Form::new().part("", reqwest::multipart::Part::bytes(bytes));
        reqwest::Client::new()
            .post(&url)
            .bearer_auth(&self.config.service_role)
            .multipart(upload_img_form)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let img_url = format!(
            "{}/storage/v1/object/images/{}",
            self.config.api_url, file_path
        );
        Ok(img_url)
    }
}
