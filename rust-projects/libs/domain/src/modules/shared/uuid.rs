use super::errors::UnexpectedError;

pub fn parse_str(val: &str) -> Result<String, UnexpectedError> {
    let result = uuid::Uuid::parse_str(val).map_err(|_| UnexpectedError::new("user id is not uuid".to_string()))?;
    Ok(result.to_string())
}
