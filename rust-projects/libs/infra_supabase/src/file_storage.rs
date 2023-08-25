mod user;

use std::sync::Arc;

use integrator::file_storage::user_accessor::{UserMutations, UserQueries};
pub use user::UserAccessor;

pub struct FileStorageImpl {
    user_queries: Arc<dyn UserQueries>,
    user_mutations: Arc<dyn UserMutations>,
}

impl FileStorageImpl {
    pub fn new(user_queries: Arc<dyn UserQueries>, user_mutations: Arc<dyn UserMutations>) -> Self {
        Self {
            user_queries,
            user_mutations,
        }
    }
}

impl integrator::FileStorage for FileStorageImpl {
    fn user_queries(&self) -> &Arc<dyn UserQueries> {
        &self.user_queries
    }

    fn user_mutations(&self) -> &Arc<dyn UserMutations> {
        &self.user_mutations
    }
}
