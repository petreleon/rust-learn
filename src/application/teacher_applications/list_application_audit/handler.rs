use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::teacher_applications::{
    list_application_audit::{
        TeacherApplicationAuditError, TeacherApplicationAuditQuery, TeacherApplicationAuditStore,
    },
    TeacherApplicationAuditEventOutput,
};

const REVIEW_TEACHER_APPLICATIONS: &str = "REVIEW_TEACHER_APPLICATIONS";

pub async fn list_application_audit(
    store: &mut impl TeacherApplicationAuditStore,
    query: TeacherApplicationAuditQuery,
) -> Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError> {
    if !can_platform_review_action(store, query.actor_user_id).await? {
        return Err(TeacherApplicationAuditError::PermissionDenied(
            REVIEW_TEACHER_APPLICATIONS.to_string(),
        ));
    }

    store.list_audit_events(query.application_id).await
}

async fn can_platform_review_action(
    store: &mut impl AccessDecisionStore<Error = TeacherApplicationAuditError>,
    actor_user_id: i32,
) -> Result<bool, TeacherApplicationAuditError> {
    store
        .can(
            AccessActor::user(actor_user_id),
            AccessAction::permission(REVIEW_TEACHER_APPLICATIONS),
            AccessScope::platform(),
        )
        .await
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use futures::future::{BoxFuture, FutureExt};

    use super::*;

    #[derive(Default)]
    struct FakeStore {
        can_review: bool,
        loaded_application_id: Option<i64>,
    }

    impl AccessDecisionStore for FakeStore {
        type Error = TeacherApplicationAuditError;

        fn can(
            &mut self,
            _: AccessActor,
            action: AccessAction,
            scope: AccessScope,
        ) -> BoxFuture<'_, Result<bool, TeacherApplicationAuditError>> {
            assert_eq!(action.permission_name(), REVIEW_TEACHER_APPLICATIONS);
            assert!(matches!(scope, AccessScope::Platform(_)));
            async move { Ok(self.can_review) }.boxed()
        }
    }

    impl TeacherApplicationAuditStore for FakeStore {
        fn list_audit_events(
            &mut self,
            application_id: i64,
        ) -> BoxFuture<
            '_,
            Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>,
        > {
            self.loaded_application_id = Some(application_id);
            async move { Ok(vec![audit_event(application_id)]) }.boxed()
        }
    }

    #[tokio::test]
    async fn lists_audit_events_when_actor_can_review() {
        let mut store = FakeStore {
            can_review: true,
            loaded_application_id: None,
        };

        let output = list_application_audit(&mut store, query()).await.unwrap();

        assert_eq!(output.len(), 1);
        assert_eq!(store.loaded_application_id, Some(42));
    }

    #[tokio::test]
    async fn denies_actor_without_review_permission() {
        let mut store = FakeStore::default();

        let error = list_application_audit(&mut store, query())
            .await
            .unwrap_err();

        assert!(matches!(
            error,
            TeacherApplicationAuditError::PermissionDenied(permission)
                if permission == "REVIEW_TEACHER_APPLICATIONS"
        ));
        assert_eq!(store.loaded_application_id, None);
    }

    fn query() -> TeacherApplicationAuditQuery {
        TeacherApplicationAuditQuery {
            actor_user_id: 7,
            application_id: 42,
        }
    }

    fn audit_event(application_id: i64) -> TeacherApplicationAuditEventOutput {
        TeacherApplicationAuditEventOutput {
            actor_user_id: Some(7),
            application_id,
            created_at: Utc::now(),
            event_type: "submitted".to_string(),
            from_status: None,
            id: 1,
            reason: None,
            to_status: "submitted".to_string(),
        }
    }
}
