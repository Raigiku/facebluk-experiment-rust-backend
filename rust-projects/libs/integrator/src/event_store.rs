use self::user_accessor::{UserMutations, UserQueries};

pub mod user_accessor;

pub trait EventStore: Sync + Send + UserQueries + UserMutations {}
