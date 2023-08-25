use std::{future::Future, pin::Pin};

use actix_web::{web, FromRequest, HttpRequest};
use domain::{
    config::Environment,
    modules::shared::{errors::UnexpectedError, jsonwebtoken, uuid},
};
use serde::Deserialize;

use super::{app_state::AppState, error_handling::FaceblukHttpError};

pub struct UserExtractor {
    pub user_id: String,
}

impl FromRequest for UserExtractor {
    type Error = FaceblukHttpError;

    type Future = Pin<Box<dyn Future<Output = Result<UserExtractor, FaceblukHttpError>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let app_state = req.app_data::<web::Data<AppState>>().unwrap();
        if app_state.shared_config.environment == Environment::Local {
            let user = req
                .headers()
                .get("authorization")
                .ok_or(UnexpectedError::new(
                    "authorization header not found".to_string(),
                ))
                .and_then(|header| {
                    header.to_str().map_err(|_err| {
                        UnexpectedError::new("header cant parse to str".to_string())
                    })
                })
                .and_then(|header_str| uuid::parse_str(header_str))
                .map(|user_id| UserExtractor {
                    user_id: user_id.to_string(),
                });

            Box::pin(async { Ok(user?) })
        } else {
            let user = req
                .headers()
                .get("authorization")
                .ok_or(UnexpectedError::new(
                    "authorization header not found".to_string(),
                ))
                .and_then(|header| {
                    header.to_str().map_err(|_err| {
                        UnexpectedError::new("header cant parse to str".to_string())
                    })
                })
                .and_then(|header_str| {
                    if header_str.starts_with("Bearer ") {
                        Ok(header_str.replace("Bearer ", ""))
                    } else {
                        Err(UnexpectedError::new(
                            "bearer token has wrong format".to_string(),
                        ))
                    }
                })
                .and_then(|header_str| {
                    jsonwebtoken::decode::<JwtClaims>(&header_str, app_state.api_config.auth_jwt_secret.as_ref())
                })
                .map(|jwt| UserExtractor {
                    user_id: jwt.sub,
                });

            Box::pin(async { Ok(user?) })
        }
    }
}

#[derive(Deserialize)]
pub struct JwtClaims {
    pub sub: String,
}

// aud: string
//   exp: number
//
//   email: string
//   phone: string
//   app_metadata: {
//     provider: string
//     providers: string[]
//   }
//   user_metadata: {
//     avatar_url: string
//     email: string
//     email_verified: boolean
//     full_name: string
//     iss: string
//     name: string
//     picture: string
//     provider_id: string
//     sub: string
//     registeredAt?: Date
//   }
//   role: string
//   aal: string
//   amr: {
//     method: string
//     timestamp: number
//   }[]
//   session_id: string
