use std::env;

pub struct Config {
    pub url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            url: env::var("RABBITMQ_URL").unwrap().parse().unwrap(),
        }
    }
}
