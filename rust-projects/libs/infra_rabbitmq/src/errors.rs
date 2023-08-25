use domain::modules::shared::errors::UnexpectedError;

pub fn map_lapin_error(err: lapin::Error) -> UnexpectedError {
    UnexpectedError::new(err.to_string())
}