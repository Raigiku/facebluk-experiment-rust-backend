use serde::Serialize;

pub const REGISTER_USER_MSG_KEY: &str = "register-user";

#[derive(Serialize)]
pub struct RegisterUser {
    pub name: String,
    pub alias: String,
    pub profile_picture_url: Option<String>,
}
