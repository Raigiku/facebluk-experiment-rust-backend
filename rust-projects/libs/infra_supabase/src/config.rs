use std::env;

pub struct Config {
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: env::var("SUPABASE_API_URL").unwrap().parse().unwrap(),
        }
    }
}
