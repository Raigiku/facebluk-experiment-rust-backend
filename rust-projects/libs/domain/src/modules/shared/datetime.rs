use chrono::Utc;

use super::errors::UnexpectedError;

pub struct DateTime(chrono::DateTime<Utc>);

impl TryFrom<String> for DateTime {
    type Error = UnexpectedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let result = chrono::DateTime::parse_from_rfc3339(&value)
            .map_err(|err| UnexpectedError::new(err.to_string()))?;
        Ok(DateTime(result.with_timezone(&Utc)))
    }
}
