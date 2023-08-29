use crate::map_unexpected_error;

use super::errors::UnexpectedError;

pub fn parse_str(val: &str) -> Result<String, UnexpectedError> {
    let result = uuid::Uuid::parse_str(val).map_err(|err| map_unexpected_error!(err))?;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_str;

    #[test]
    fn test_parse_str() {
        let val = "2d0c40a1-88aa-4554-ae93-ae601beae841";
        let res = parse_str(val);
        assert_eq!(res.is_ok(), true);
    }
}
