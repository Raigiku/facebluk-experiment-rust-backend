use std::env;

pub struct Config {
    pub host: String,
    pub db_name: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: env::var("EVENT_STORE_HOST").unwrap().parse().unwrap(),
            db_name: env::var("EVENT_STORE_DB").unwrap().parse().unwrap(),
            username: env::var("EVENT_STORE_USERNAME").unwrap().parse().unwrap(),
            password: env::var("EVENT_STORE_PASSWORD").unwrap().parse().unwrap(),
            port: env::var("EVENT_STORE_PORT").unwrap().parse().unwrap(),
        }
    }
}
