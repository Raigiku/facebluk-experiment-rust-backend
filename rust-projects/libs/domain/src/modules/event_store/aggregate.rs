use crate::modules::shared::datetime::DateTime;

pub struct Aggregate {
    id: String,
    version: i64,
    created_at: DateTime,
}

impl Aggregate {
    pub fn from_existing(id: String, version: i64, created_at: DateTime) -> Self {
        Self {
            id,
            version,
            created_at,
        }
    }
}
