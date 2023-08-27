use crate::modules::shared::datetime::DateTime;

pub struct EventData {
    pub aggregate_id: String,
    pub aggregate_version: i64,
    pub created_at: DateTime,
    pub published: bool,
}

impl EventData {
    pub fn new(aggregate_id: String) -> Self {
        Self {
            aggregate_id,
            aggregate_version: 1,
            created_at: DateTime::now(),
            published: false,
        }
    }
}
