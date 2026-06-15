use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};

use super::*;
use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewAuditSummaryOutput,
    TeacherApplicationPlatformReviewCourseOutput, TeacherApplicationPlatformReviewDataset,
    TeacherApplicationPlatformReviewOrganizationOutput,
    TeacherApplicationPlatformReviewSummaryOutput, TeacherApplicationPlatformReviewUserOutput,
};
use crate::domain::access_control::permissions::Permissions;

#[derive(Default)]
struct FakeStore {
    can_review: bool,
    can_approve: bool,
    can_reject: bool,
    listed: bool,
}

impl AccessDecisionStore for FakeStore {
    type Error = TeacherApplicationPlatformReviewError;

    fn can(
        &mut self,
        _: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>> {
        assert!(matches!(scope, AccessScope::Platform(_)));
        let permission = action.permission_name();
        let allowed = if permission == Permissions::REVIEW_TEACHER_APPLICATIONS.to_string() {
            self.can_review
        } else if permission == Permissions::APPROVE_TEACHER_APPLICATION.to_string() {
            self.can_approve
        } else if permission == Permissions::REJECT_TEACHER_APPLICATION.to_string() {
            self.can_reject
        } else {
            panic!("unexpected platform-review permission {permission}");
        };
        async move { Ok(allowed) }.boxed()
    }
}

impl TeacherApplicationPlatformReviewStore for FakeStore {
    fn list_applications(
        &mut self,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError>,
    > {
        self.listed = true;
        async move {
            Ok(TeacherApplicationPlatformReviewDataset {
                applications: vec![
                    application(1, "submitted", "Async mentor", "Ada"),
                    application(2, "approved", "Archived profile", "Grace"),
                ],
                summary: TeacherApplicationPlatformReviewSummaryOutput {
                    submitted: 1,
                    approved: 1,
                    total: 2,
                    ..Default::default()
                },
            })
        }
        .boxed()
    }
}

#[tokio::test]
async fn denies_actor_without_review_permission() {
    let mut store = FakeStore::default();

    let error = list_platform_review_applications(
        &mut store,
        TeacherApplicationPlatformReviewQuery {
            actor_user_id: 7,
            ..Default::default()
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(
        error,
        TeacherApplicationPlatformReviewError::PermissionDenied(permission)
            if permission == "REVIEW_TEACHER_APPLICATIONS"
    ));
    assert!(!store.listed);
}

#[tokio::test]
async fn filters_searches_and_pages_review_queue() {
    let mut store = FakeStore {
        can_review: true,
        can_approve: true,
        can_reject: false,
        listed: false,
    };

    let output = list_platform_review_applications(
        &mut store,
        TeacherApplicationPlatformReviewQuery {
            actor_user_id: 7,
            status: Some(" SUBMITTED ".to_string()),
            search: Some(" async ".to_string()),
            limit: Some(1),
            offset: Some(0),
        },
    )
    .await
    .unwrap();

    assert_eq!(output.total, 1);
    assert_eq!(output.applications[0].id, 1);
    assert_eq!(output.status.as_deref(), Some("submitted"));
    assert_eq!(output.search.as_deref(), Some("async"));
    assert_eq!(output.summary.total, 2);
    assert!(output.operator_permissions.can_view_applications);
    assert!(output.operator_permissions.can_approve_applications);
    assert!(!output.operator_permissions.can_reject_applications);
    assert!(output.operator_permissions.can_request_changes);
}

fn application(
    id: i64,
    status: &str,
    experience_summary: &str,
    applicant_name: &str,
) -> TeacherApplicationPlatformReviewItemOutput {
    let now = Utc::now();
    TeacherApplicationPlatformReviewItemOutput {
        id,
        applicant: TeacherApplicationPlatformReviewUserOutput {
            id: id as i32,
            name: applicant_name.to_string(),
            email: format!("{applicant_name}@example.test"),
        },
        requested_scope: "course".to_string(),
        requested_organization: Some(TeacherApplicationPlatformReviewOrganizationOutput {
            id: 33,
            name: "Org".to_string(),
        }),
        requested_course: Some(TeacherApplicationPlatformReviewCourseOutput {
            id: 44,
            title: "Course".to_string(),
        }),
        sponsor_organization: None,
        experience_summary: experience_summary.to_string(),
        portfolio_links: Vec::new(),
        status: status.to_string(),
        reviewer: None,
        decision_reason: None,
        audit: TeacherApplicationPlatformReviewAuditSummaryOutput::default(),
        created_at: now,
        updated_at: now,
        decided_at: None,
    }
}
