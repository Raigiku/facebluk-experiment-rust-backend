use domain::modules::shared::errors::UnexpectedError;

pub fn map_sqlx_error(err: sqlx::Error) -> UnexpectedError {
    UnexpectedError::new(err.to_string())
}