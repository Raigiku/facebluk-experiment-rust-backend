use serde::Serialize;

use crate::map_unexpected_error;

use super::errors::UnexpectedError;

pub fn serialize<T: Serialize>(value: &T) -> Result<String, UnexpectedError> {
    serde_json::to_string(value).map_err(|err| map_unexpected_error!(err))
}
