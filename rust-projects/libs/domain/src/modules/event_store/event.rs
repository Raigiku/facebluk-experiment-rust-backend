use crate::modules::shared::datetime::DateTime;

pub struct Event {
    aggregate_id: String,
    aggregate_version: i64,
    created_at: DateTime,
    published: bool,
}
