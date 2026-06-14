use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::assign_platform_role::{
    self, AssignPlatformRoleCommand, AssignPlatformRoleError, AssignPlatformRoleOutcome,
    PlatformRoleAssignmentUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::identity::platform_role_assignment_store::PostgresPlatformRoleAssignmentStore;
use crate::utils::notifications::NotificationsState;

#[derive(Clone)]
pub struct PostgresPlatformRoleAssignmentUseCase {
    pool: DbPool,
    notifications: NotificationsState,
}

impl PostgresPlatformRoleAssignmentUseCase {
    pub fn new(pool: DbPool, notifications: NotificationsState) -> Self {
        Self {
            pool,
            notifications,
        }
    }
}

impl PlatformRoleAssignmentUseCase for PostgresPlatformRoleAssignmentUseCase {
    fn assign_platform_role(
        &self,
        command: AssignPlatformRoleCommand,
    ) -> BoxFuture<'_, Result<AssignPlatformRoleOutcome, AssignPlatformRoleError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformRoleAssignmentStore::new(&mut conn);
            let outcome = assign_platform_role::assign_platform_role(&mut store, command).await?;
            self.notify_assignment(&outcome).await;
            Ok(outcome)
        }
        .boxed()
    }
}

impl PostgresPlatformRoleAssignmentUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        AssignPlatformRoleError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| AssignPlatformRoleError::Connection(error.to_string()))
    }

    async fn notify_assignment(&self, outcome: &AssignPlatformRoleOutcome) {
        if let Err(error) = self
            .notifications
            .send_role_assignment_notification(
                outcome.target_user_id,
                "platform",
                None,
                &outcome.role_name,
            )
            .await
        {
            log::warn!(
                "event=notification_send_failed kind=role_assignment scope=platform target_user_id={} error={:?}",
                outcome.target_user_id,
                error
            );
        }
    }
}
