use async_trait::async_trait;
use integrator::file_storage::user_accessor::UserMutations;
use serde::Deserialize;

use super::UserAccessor;

#[async_trait]
impl UserMutations for UserAccessor {
    async fn upload_image(&self, user_id: &str, file_name: &str, bytes: Vec<u8>) -> String {
        let url = format!("{}/file-storage", self.config.api_url);
        let file_path = format!("users/{}/{}", user_id, file_name);
        let http_client = reqwest::Client::new();

        let upload_img_form = reqwest::multipart::Form::new()
            .text("function", "upload_file")
            .text("bucket_name", "images")
            .text("file_path", file_path.clone())
            .part("file_bytes", reqwest::multipart::Part::bytes(bytes));
        http_client
            .post(&url)
            .multipart(upload_img_form)
            .send()
            .await
            .unwrap();

        let query_params = &[
            ("function", "find_public_file_url"),
            ("bucket_name", "images"),
            ("file_path", &file_path),
        ];
        let http_res = http_client
            .get(&url)
            .query(query_params)
            .send()
            .await
            .unwrap();
        let res_dto = http_res.json::<UrlDTO>().await.unwrap();
        res_dto.url
    }
}

#[derive(Deserialize)]
struct UrlDTO {
    url: String,
}
