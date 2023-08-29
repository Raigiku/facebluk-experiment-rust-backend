use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use domain::modules::shared::errors::{FaceblukError, UnexpectedError, UserExposedError};
use serde::Serialize;
use std::convert::From;

#[derive(Debug, Display, Error, Serialize)]
#[display(fmt = "")]
pub struct FaceblukHttpError {
    #[serde(skip)]
    status_code: StatusCode,
    message: String,
}

impl ResponseError for FaceblukHttpError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        if self.status_code == StatusCode::BAD_REQUEST {
            HttpResponse::build(self.status_code()).json(self)
        } else {
            HttpResponse::build(self.status_code()).finish()
        }
    }
}

impl From<FaceblukError> for FaceblukHttpError {
    fn from(value: FaceblukError) -> Self {
        match value {
            FaceblukError::UserExposed(val) => Self {
                status_code: StatusCode::BAD_REQUEST,
                message: val.msg,
            },
            FaceblukError::Unexpected(val) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: val.msg,
            },
        }
    }
}

impl From<UserExposedError> for FaceblukHttpError {
    fn from(value: UserExposedError) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: value.msg,
        }
    }
}

impl From<UnexpectedError> for FaceblukHttpError {
    fn from(value: UnexpectedError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: value.msg,
        }
    }
}
