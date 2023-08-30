use async_trait::async_trait;
use domain::{map_unexpected_error, modules::shared::errors::UnexpectedError};
use integrator::file_storage::file_accessor::FileMutations;

use super::FileStorageImpl;

#[async_trait]
impl FileMutations for FileStorageImpl {
    async fn m_upload_file(
        &self,
        file_path: &str,
        bytes: Vec<u8>,
    ) -> Result<String, UnexpectedError> {
        let url = format!("{}/storage/v1/object/{}", self.config.api_url, file_path);
        let upload_img_form =
            reqwest::multipart::Form::new().part("", reqwest::multipart::Part::bytes(bytes));

        reqwest::Client::new()
            .post(&url)
            .bearer_auth(&self.config.service_role)
            .multipart(upload_img_form)
            .send()
            .await
            .map_err(|err| map_unexpected_error!(err))?;

        let img_url = format!("{}/storage/v1/object/{}", self.config.api_url, file_path);
        Ok(img_url)
    }
}
