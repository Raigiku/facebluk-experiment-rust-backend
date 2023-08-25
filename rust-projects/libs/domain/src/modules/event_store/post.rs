pub struct Post {
    user_id: String,
    description: String,
    tagged_user_ids: Vec<String>,
}

impl Post {
    pub fn create(user_id: String, description: String, tagged_user_ids: Vec<String>) -> Self {
        return Self {
            user_id,
            description,
            tagged_user_ids,
        };
    }
}

pub mod events {
    pub enum Event {}
    pub struct Created {
        payload: CreatedPayload,
    }
    pub struct CreatedPayload {
        user_id: String,
        description: String,
        tagged_user_ids: Vec<String>,
    }
}
