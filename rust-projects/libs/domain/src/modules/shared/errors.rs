use derive_more::{Display, Error};

pub enum FaceblukError {
    UserExposed(UserExposedError),
    Unexpected(UnexpectedError),
}

impl From<UnexpectedError> for FaceblukError {
    fn from(value: UnexpectedError) -> Self {
        FaceblukError::Unexpected(value)
    }
}

impl From<UserExposedError> for FaceblukError {
    fn from(value: UserExposedError) -> Self {
        FaceblukError::UserExposed(value)
    }
}

#[derive(Error, Display, Debug)]
pub struct UserExposedError {
    pub msg: String,
}

impl UserExposedError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

#[derive(Error, Display, Debug)]
pub struct UnexpectedError {
    pub msg: String,
}

impl UnexpectedError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

#[macro_export]
macro_rules! map_unexpected_error {
    ($err:expr) => {
        UnexpectedError::new(format!("file: {} / line: {} / error: {:?}", file!(), line!(), $err))
    };
}