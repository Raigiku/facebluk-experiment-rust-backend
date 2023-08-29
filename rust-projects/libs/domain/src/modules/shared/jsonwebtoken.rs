use crate::map_unexpected_error;

use super::errors::UnexpectedError;
use serde::de::DeserializeOwned;

pub fn decode<T: DeserializeOwned>(val: &str, secret: &str) -> Result<T, UnexpectedError> {
    let result = jsonwebtoken::decode::<T>(
        &val,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|err| map_unexpected_error!(err))?;
    Ok(result.claims)
}
