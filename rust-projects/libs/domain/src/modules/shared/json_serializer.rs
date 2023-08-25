use serde::Serialize;

use super::errors::UnexpectedError;

pub fn serialize<T: Serialize>(value: &T) -> Result<String, UnexpectedError> {
    serde_json::to_string(value).map_err(|err| UnexpectedError::new(err.to_string()))
}
