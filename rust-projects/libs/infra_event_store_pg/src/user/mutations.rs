use async_trait::async_trait;
use integrator::event_store::user_accessor::UserMutations;

use super::UserAccessor;

#[async_trait]
impl UserMutations for UserAccessor {}
