use std::sync::Arc;

use domain::modules::{
    event_store::user::User as EsUser,
    msg_broker::user::RegisterUser,
    shared::{datetime::DateTime, errors::UnexpectedError},
};

use crate::{EventStore, UserAuth};

pub async fn handle(
    req: RegisterUser,
    event_store: Arc<dyn EventStore>,
    user_auth: Arc<dyn UserAuth>,
) -> Result<(), UnexpectedError> {
    let es_user = event_store.user_queries().find_by_id(&req.user_id).await?;
    if es_user.is_none() {
        let user_registered = EsUser::register(
            &req.user_id,
            &req.name,
            &req.alias,
            &req.profile_picture_url,
        );
        event_store
            .user_mutations()
            .register(&user_registered)
            .await?;
    }

    let ua_user = user_auth.user_queries().find_by_id(&req.user_id).await?;
    if let Some(ref user) = ua_user {
        if user.is_registered() {
            user_auth
                .user_mutations()
                .mark_as_registered(&req.user_id, &DateTime::now())
                .await?;
        }
    }

    Ok(())
}
