use domain::modules::shared::errors::UnexpectedError;
use reqwest::StatusCode;

pub fn map_reqwest_error(err: reqwest::Error) -> UnexpectedError {
    UnexpectedError::new(err.to_string())
}

pub fn map_http_status_error(code: &StatusCode, response_body: &str) -> UnexpectedError {
    UnexpectedError::new(format!(
        "http_status_error:code={};body={}",
        code, response_body
    ))
}
