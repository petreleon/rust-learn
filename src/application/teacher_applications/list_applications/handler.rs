use crate::application::teacher_applications::{
    list_applications::{
        TeacherApplicationListError, TeacherApplicationListFilter, TeacherApplicationListQuery,
        TeacherApplicationListStore,
    },
    TeacherApplicationOutput,
};
use crate::domain::teacher_applications::status::normalize_optional_status;

pub async fn list_applications(
    store: &mut impl TeacherApplicationListStore,
    query: TeacherApplicationListQuery,
) -> Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError> {
    if !store
        .can_review_teacher_applications(query.actor_user_id)
        .await?
    {
        return Err(TeacherApplicationListError::PermissionDenied(
            "REVIEW_TEACHER_APPLICATIONS".to_string(),
        ));
    }

    store
        .list_applications(TeacherApplicationListFilter {
            applicant_user_id: query.applicant_user_id,
            limit: query.limit,
            offset: query.offset,
            organization_sponsor_id: query.organization_sponsor_id,
            status: normalize_optional_status(query.status)?,
        })
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
        filter: Option<TeacherApplicationListFilter>,
    }

    impl TeacherApplicationListStore for FakeStore {
        fn can_review_teacher_applications(
            &mut self,
            _: i32,
        ) -> BoxFuture<'_, Result<bool, TeacherApplicationListError>> {
            async move { Ok(self.can_review) }.boxed()
        }

        fn list_applications(
            &mut self,
            filter: TeacherApplicationListFilter,
        ) -> BoxFuture<'_, Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError>>
        {
            self.filter = Some(filter);
            async move { Ok(vec![application()]) }.boxed()
        }
    }

    #[tokio::test]
    async fn normalizes_filter_before_listing() {
        let mut store = FakeStore {
            can_review: true,
            filter: None,
        };

        let output = list_applications(
            &mut store,
            TeacherApplicationListQuery {
                actor_user_id: 7,
                applicant_user_id: Some(8),
                status: Some(" SUBMITTED ".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(output.len(), 1);
        let filter = store.filter.expect("filter should be passed to store");
        assert_eq!(filter.status.as_deref(), Some("submitted"));
        assert_eq!(filter.applicant_user_id, Some(8));
    }

    #[tokio::test]
    async fn denies_actor_without_review_permission() {
        let mut store = FakeStore::default();

        let error = list_applications(
            &mut store,
            TeacherApplicationListQuery {
                actor_user_id: 7,
                ..Default::default()
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(
            error,
            TeacherApplicationListError::PermissionDenied(permission)
                if permission == "REVIEW_TEACHER_APPLICATIONS"
        ));
        assert!(store.filter.is_none());
    }

    fn application() -> TeacherApplicationOutput {
        let now = Utc::now();
        TeacherApplicationOutput {
            applicant_user_id: 8,
            created_at: now,
            decided_at: None,
            decision_reason: None,
            experience_summary: "Mentor".to_string(),
            id: 1,
            idempotency_key: None,
            organization_sponsor_id: None,
            portfolio_links: serde_json::json!([]),
            requested_course_id: None,
            requested_organization_id: None,
            requested_scope: "platform".to_string(),
            reviewer_id: None,
            status: "submitted".to_string(),
            updated_at: now,
        }
    }
}
