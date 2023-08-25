use std::{env, str::FromStr};

use crate::modules::shared::errors::UnexpectedError;

pub struct SharedConfig {
    pub environment: Environment,
}

impl SharedConfig {
    pub fn new() -> Self {
        Self {
            environment: env::var("ENVIRONMENT").unwrap().parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Environment {
    Local,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = UnexpectedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("local") {
            Ok(Self::Local)
        } else if s.eq_ignore_ascii_case("staging") {
            Ok(Self::Staging)
        } else if s.eq_ignore_ascii_case("production") {
            Ok(Self::Production)
        } else {
            Err(UnexpectedError::new(
                "unknown environment env var".to_string(),
            ))
        }
    }
}
