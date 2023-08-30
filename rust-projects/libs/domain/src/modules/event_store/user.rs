use crate::modules::shared::errors::ValidationError;

use super::{aggregate::Aggregate, event_data::EventData};

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
        id: &str,
        name: &str,
        alias: &str,
        profile_picture_url: &Option<String>,
    ) -> events::Registered {
        events::Registered {
            data: EventData::new(id.to_string()),
            payload: events::RegisteredPayload {
                name: name.to_string(),
                alias: alias.to_string(),
                profile_picture_url: profile_picture_url.clone(),
            },
        }
    }

    pub fn update_info(
        id: &str,
        name: &Option<String>,
        profile_picture_url: &Option<String>,
    ) -> events::InfoUpdated {
        events::InfoUpdated {
            data: EventData::new(id.to_string()),
            payload: events::InfoUpdatedPayload {
                name: name.clone(),
                profile_picture_url: profile_picture_url.clone(),
            },
        }
    }
}

pub mod events {
    use crate::modules::event_store::event_data::EventData;

    pub enum Event {
        Registered(Registered),
        InfoUpdated(InfoUpdated),
    }

    pub struct Registered {
        pub data: EventData,
        pub payload: RegisteredPayload,
    }
    pub struct RegisteredPayload {
        pub name: String,
        pub alias: String,
        pub profile_picture_url: Option<String>,
    }

    pub struct InfoUpdated {
        pub data: EventData,
        pub payload: InfoUpdatedPayload,
    }
    pub struct InfoUpdatedPayload {
        pub name: Option<String>,
        pub profile_picture_url: Option<String>,
    }
}

const NAME_MAX_LENGTH: usize = 100;
pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.len() > NAME_MAX_LENGTH {
        Err(ValidationError::new(format!(
            "name max length {NAME_MAX_LENGTH}"
        )))
    } else if name.is_empty() {
        Err(ValidationError::new(format!("name cannot be empty")))
    } else {
        Ok(())
    }
}

const ALIAS_MAX_LENGTH: usize = 10;
pub fn validate_alias(alias: &str) -> Result<(), ValidationError> {
    if alias.len() > ALIAS_MAX_LENGTH {
        Err(ValidationError::new(format!(
            "alias max length {ALIAS_MAX_LENGTH}"
        )))
    } else if alias.is_empty() {
        Err(ValidationError::new(format!("alias cannot be empty")))
    } else if !alias.chars().all(|x| x.is_alphanumeric()) {
        Err(ValidationError::new(format!(
            "alias needs to be alphanumeric"
        )))
    } else {
        Ok(())
    }
}
