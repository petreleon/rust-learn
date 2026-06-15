use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionError, AccessDecisionUseCase, AccessScope,
};
use crate::db::DbPool;
use crate::infra::postgres::access_control::permission_checks;

impl AccessDecisionUseCase for DbPool {
    fn can(
        &self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, AccessDecisionError>> {
        async move {
            let mut conn = self
                .get()
                .await
                .map_err(|error| AccessDecisionError::Connection(error.to_string()))?;

            permission_checks::can(&mut conn, actor, action, scope)
                .await
                .map_err(|error| AccessDecisionError::Query(error.to_string()))
        }
        .boxed()
    }
}
