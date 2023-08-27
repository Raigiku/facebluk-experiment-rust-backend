use domain::modules::shared::errors::UnexpectedError;

pub fn map_sqlx_error(err: sqlx::Error, file: &str, line: u32) -> UnexpectedError {
    UnexpectedError::new(format!(
        "file: {} / line: {} / error: {:?}",
        file, line, err
    ))
}
