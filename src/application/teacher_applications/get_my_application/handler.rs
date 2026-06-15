use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationSelfError, TeacherApplicationSelfOutput, TeacherApplicationSelfStore,
};

pub async fn get_my_application(
    store: &mut impl TeacherApplicationSelfStore,
    actor_user_id: i32,
) -> Result<TeacherApplicationSelfOutput, TeacherApplicationSelfError> {
    let application = store
        .find_latest_application_for_applicant(actor_user_id)
        .await?;
    let audit_events = match application.as_ref() {
        Some(application) => store.list_audit_events(application.id).await?,
        None => Vec::new(),
    };

    Ok(TeacherApplicationSelfOutput {
        application,
        audit_events,
    })
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use futures::future::{BoxFuture, FutureExt};

    use super::*;
    use crate::application::teacher_applications::get_my_application::{
        TeacherApplicationAuditEventOutput, TeacherApplicationOutput,
    };
    use crate::domain::teacher_applications::{
        audit::TeacherApplicationAuditEventType, portfolio::portfolio_links_from_urls,
    };

    #[derive(Default)]
    struct FakeStore {
        application: Option<TeacherApplicationOutput>,
        audit_loaded_for: Option<i64>,
    }

    impl TeacherApplicationSelfStore for FakeStore {
        fn find_latest_application_for_applicant(
            &mut self,
            _: i32,
        ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSelfError>>
        {
            async move { Ok(self.application.clone()) }.boxed()
        }

        fn list_audit_events(
            &mut self,
            application_id: i64,
        ) -> BoxFuture<
            '_,
            Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationSelfError>,
        > {
            self.audit_loaded_for = Some(application_id);
            async move { Ok(vec![audit_event(application_id)]) }.boxed()
        }
    }

    #[tokio::test]
    async fn returns_application_with_its_audit_events() {
        let mut store = FakeStore {
            application: Some(application(12)),
            audit_loaded_for: None,
        };

        let output = get_my_application(&mut store, 7).await.unwrap();

        assert_eq!(output.application.as_ref().map(|item| item.id), Some(12));
        assert_eq!(output.audit_events.len(), 1);
        assert_eq!(store.audit_loaded_for, Some(12));
    }

    #[tokio::test]
    async fn skips_audit_lookup_when_user_has_no_application() {
        let mut store = FakeStore::default();

        let output = get_my_application(&mut store, 7).await.unwrap();

        assert!(output.application.is_none());
        assert!(output.audit_events.is_empty());
        assert_eq!(store.audit_loaded_for, None);
    }

    fn application(id: i64) -> TeacherApplicationOutput {
        let now = Utc::now();
        TeacherApplicationOutput {
            applicant_user_id: 7,
            created_at: now,
            decided_at: None,
            decision_reason: None,
            experience_summary: "Mentor".to_string(),
            id,
            idempotency_key: None,
            organization_sponsor_id: None,
            portfolio_links: portfolio_links_from_urls(Vec::new()),
            requested_course_id: None,
            requested_organization_id: None,
            requested_scope: "platform".to_string(),
            reviewer_id: None,
            status: "submitted".to_string(),
            updated_at: now,
        }
    }

    fn audit_event(application_id: i64) -> TeacherApplicationAuditEventOutput {
        TeacherApplicationAuditEventOutput {
            actor_user_id: Some(7),
            application_id,
            created_at: Utc::now(),
            event_type: TeacherApplicationAuditEventType::Submitted,
            from_status: None,
            id: 20,
            reason: None,
            to_status: "submitted".to_string(),
        }
    }
}
