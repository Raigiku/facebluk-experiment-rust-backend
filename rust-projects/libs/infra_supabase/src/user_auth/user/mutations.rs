use async_trait::async_trait;
use integrator::user_auth::user_accessor::UserMutations;

use super::UserAccessor;

#[async_trait]
impl UserMutations for UserAccessor {}
