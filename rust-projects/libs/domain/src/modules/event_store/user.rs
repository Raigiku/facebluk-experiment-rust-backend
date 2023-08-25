use crate::modules::shared::errors::UserExposedError;

use super::aggregate::Aggregate;

pub struct User {
    aggregate: Aggregate,
    name: String,
    alias: String,
    profile_picture_url: Option<String>,
}

impl User {
    pub fn from_existing(
        aggregate: Aggregate,
        name: String,
        alias: String,
        profile_picture_url: Option<String>,
    ) -> Self {
        Self {
            aggregate,
            name,
            alias,
            profile_picture_url,
        }
    }

    pub fn register(
        name: String,
        alias: String,
        profile_picture_url: Option<String>,
    ) -> events::Registered {
        events::Registered {
            payload: events::RegisteredPayload {
                name,
                alias,
                profile_picture_url,
            },
        }
    }

    pub fn update_info(
        name: Option<String>,
        profile_picture_url: Option<String>,
    ) -> events::InfoUpdated {
        events::InfoUpdated {
            payload: events::InfoUpdatedPayload {
                name,
                profile_picture_url,
            },
        }
    }
}

pub mod events {
    pub enum Event {
        Registered(Registered),
        InfoUpdated(InfoUpdated),
    }

    pub struct Registered {
        pub payload: RegisteredPayload,
    }
    pub struct RegisteredPayload {
        pub name: String,
        pub alias: String,
        pub profile_picture_url: Option<String>,
    }

    pub struct InfoUpdated {
        pub payload: InfoUpdatedPayload,
    }
    pub struct InfoUpdatedPayload {
        pub name: Option<String>,
        pub profile_picture_url: Option<String>,
    }
}

const NAME_MAX_LENGTH: usize = 100;
pub fn validate_name(name: &str) -> Result<(), UserExposedError> {
    if name.len() > NAME_MAX_LENGTH {
        Err(UserExposedError::new(format!(
            "name max length {NAME_MAX_LENGTH}"
        )))
    } else if name.is_empty() {
        Err(UserExposedError::new(format!("name cannot be empty")))
    } else {
        Ok(())
    }
}

const ALIAS_MAX_LENGTH: usize = 100;
pub fn validate_alias(alias: &str) -> Result<(), UserExposedError> {
    if alias.len() > ALIAS_MAX_LENGTH {
        Err(UserExposedError::new(format!(
            "alias max length {ALIAS_MAX_LENGTH}"
        )))
    } else if alias.is_empty() {
        Err(UserExposedError::new(format!("alias cannot be empty")))
    } else {
        Ok(())
    }
}
