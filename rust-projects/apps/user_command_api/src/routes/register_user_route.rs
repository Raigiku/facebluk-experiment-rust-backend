use std::sync::Arc;

use crate::shared::{
    app_state::AppState, error_handling::FaceblukHttpError, parsers::parse_form_image,
    user_extractor::UserExtractor,
};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};
use domain::modules::{
    event_store, msg_broker,
    shared::{
        self,
        errors::{FaceblukError, UserExposedError},
        image::Image,
        json_serializer,
    },
};
use integrator::{EventStore, UserAuth};

#[post("/users/register/v1")]
pub async fn handle(
    mut form: MultipartForm<FormData>,
    deps: web::Data<AppState>,
    user: UserExtractor,
) -> actix_web::Result<impl Responder, FaceblukHttpError> {
    let profile_picture = if let Some(ref mut image) = form.profile_picture {
        Some(parse_form_image(image)?)
    } else {
        None
    };

    validate_request(
        &form.name,
        &form.alias,
        &profile_picture,
        &user.user_id,
        &deps.user_auth,
        &deps.event_store,
    )
    .await?;

    let profile_picture_url = if let Some(profile_picture) = profile_picture {
        Some(
            deps.file_storage
                .user_mutations()
                .upload_image(
                    &user.user_id,
                    &profile_picture.file_name(),
                    profile_picture.bytes,
                )
                .await?,
        )
    } else {
        None
    };

    let msg_broker_chann = deps.msg_broker.create_channel().await?;
    let serialized_msg = json_serializer::serialize(&msg_broker::user::RegisterUser {
        name: form.name.to_string(),
        alias: form.alias.to_string(),
        profile_picture_url,
    })?;
    msg_broker_chann
        .send(msg_broker::user::REGISTER_USER_MSG_KEY, &serialized_msg)
        .await?;

    Ok(HttpResponse::Ok())
}

#[derive(MultipartForm)]
pub struct FormData {
    name: Text<String>,
    alias: Text<String>,
    profile_picture: Option<TempFile>,
}

async fn validate_request(
    name: &str,
    alias: &str,
    profile_picture: &Option<Image>,
    user_id: &str,
    user_auth: &Arc<dyn UserAuth>,
    event_store: &Arc<dyn EventStore>,
) -> Result<(), FaceblukError> {
    event_store::user::validate_name(name)?;

    event_store::user::validate_alias(alias)?;

    if let Some(profile_picture) = profile_picture {
        shared::image::validate(profile_picture)?;
    }

    let alias_exists = event_store.user_queries().alias_exists(alias).await?;
    if alias_exists {
        return Err(UserExposedError::new("user alias already exists".to_string()).into());
    }

    let user = user_auth.user_queries().find_by_id(user_id).await?;
    if user.is_none() {
        return Err(UserExposedError::new("user id does not exist".to_string()).into());
    }

    Ok(())
}
