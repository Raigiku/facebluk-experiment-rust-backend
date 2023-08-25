use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub auth_jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: env::var("USER_CMD_API_PORT").unwrap().parse().unwrap(),
            auth_jwt_secret: env::var("USER_CMD_API_JWT_SECRET").unwrap(),
        }
    }
}
