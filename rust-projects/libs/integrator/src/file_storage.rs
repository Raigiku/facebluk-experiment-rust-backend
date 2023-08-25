use std::sync::Arc;

use self::user_accessor::{UserMutations, UserQueries};

pub mod user_accessor;

pub trait FileStorage: Sync + Send {
    fn user_queries(&self) -> &Arc<dyn UserQueries>;
    fn user_mutations(&self) -> &Arc<dyn UserMutations>;
}
