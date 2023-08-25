use crate::modules::shared::datetime::DateTime;

pub struct User {
    id: String,
    registered_at: Option<DateTime>,
}

impl User {
    pub fn from_existing(id: String, registered_at: Option<DateTime>) -> Self {
        Self { id, registered_at }
    }

    pub fn is_registered(&self) -> bool {
        self.registered_at.is_some()
    }
}
