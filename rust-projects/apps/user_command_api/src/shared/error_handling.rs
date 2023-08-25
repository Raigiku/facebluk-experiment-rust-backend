use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};
use domain::modules::shared::{errors::{FaceblukError, UnexpectedError, UserExposedError}, json_serializer};
use log::error;
use serde::Serialize;
use std::convert::From;

#[derive(Debug, Display, Error, Serialize)]
#[display(fmt = "status_code: {} | msg: {}", status_code, message)]
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
        error!("{}", self.message);
        let mut res_builder = HttpResponse::build(self.status_code());
        res_builder.insert_header(ContentType::json());
        if self.status_code == StatusCode::BAD_REQUEST {
            res_builder.body(json_serializer::serialize(self).unwrap());
        }
        res_builder.finish()
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
