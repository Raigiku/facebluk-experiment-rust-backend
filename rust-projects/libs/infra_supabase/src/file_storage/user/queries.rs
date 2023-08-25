use async_trait::async_trait;
use integrator::file_storage::user_accessor::UserQueries;

use super::UserAccessor;

#[async_trait]
impl UserQueries for UserAccessor {}
