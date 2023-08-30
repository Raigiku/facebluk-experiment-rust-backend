use self::user_accessor::{UserMutations, UserQueries};

pub mod user_accessor;

pub trait UserAuth: Sync + Send + UserQueries + UserMutations {}
