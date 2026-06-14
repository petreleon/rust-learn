use crate::application::teacher_applications::{
    list_application_audit::{
        TeacherApplicationAuditError, TeacherApplicationAuditQuery, TeacherApplicationAuditStore,
    },
    TeacherApplicationAuditEventOutput,
};

pub async fn list_application_audit(
    store: &mut impl TeacherApplicationAuditStore,
    query: TeacherApplicationAuditQuery,
) -> Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError> {
    if !store
        .can_review_teacher_applications(query.actor_user_id)
        .await?
    {
        return Err(TeacherApplicationAuditError::PermissionDenied(
            "REVIEW_TEACHER_APPLICATIONS".to_string(),
        ));
    }

    store.list_audit_events(query.application_id).await
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

    impl TeacherApplicationAuditStore for FakeStore {
        fn can_review_teacher_applications(
            &mut self,
            _: i32,
        ) -> BoxFuture<'_, Result<bool, TeacherApplicationAuditError>> {
            async move { Ok(self.can_review) }.boxed()
        }

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
