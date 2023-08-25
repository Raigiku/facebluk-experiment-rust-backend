mod user;

use std::sync::Arc;

use integrator::user_auth::user_accessor::{UserMutations, UserQueries};
pub use user::UserAccessor;

pub struct UserAuthImpl {
    user_queries: Arc<dyn UserQueries>,
    user_mutations: Arc<dyn UserMutations>,
}

impl UserAuthImpl {
    pub fn new(user_queries: Arc<dyn UserQueries>, user_mutations: Arc<dyn UserMutations>) -> Self {
        Self {
            user_queries,
            user_mutations,
        }
    }

}

impl integrator::UserAuth for UserAuthImpl {
    fn user_queries(&self) -> &Arc<dyn UserQueries> {
        &self.user_queries
    }

    fn user_mutations(&self) -> &Arc<dyn UserMutations> {
        &self.user_mutations
    }
}
