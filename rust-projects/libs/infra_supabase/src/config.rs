use std::env;

pub struct Config {
    pub api_url: String,
    pub service_role: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: env::var("SUPABASE_API_URL").unwrap().parse().unwrap(),
            service_role: env::var("SUPABASE_SERVICE_ROLE").unwrap().parse().unwrap(),
        }
    }
}
