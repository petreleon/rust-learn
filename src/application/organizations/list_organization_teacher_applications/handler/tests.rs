use futures::future::{ready, BoxFuture, FutureExt};

use super::list_organization_teacher_applications;
use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationItemOutput,
    OrganizationTeacherApplicationListError, OrganizationTeacherApplicationListQuery,
    OrganizationTeacherApplicationListStore, OrganizationTeacherApplicationOrganizationOutput,
    OrganizationTeacherApplicationPermissionsOutput, TeacherApplicationDashboardSummaryOutput,
    TeacherApplicationUserSummaryOutput,
};

#[tokio::test]
async fn filters_status_search_and_paginates_after_permission() {
    let mut store = FakeStore::allowed();

    let output = list_organization_teacher_applications(
        &mut store,
        OrganizationTeacherApplicationListQuery {
            actor_user_id: 7,
            organization_id: 9,
            status: Some(" submitted ".to_string()),
            search: Some(" rust ".to_string()),
            limit: Some(1),
            offset: Some(0),
        },
    )
    .await
    .expect("list should succeed");

    assert_eq!(output.total, 1);
    assert_eq!(output.applications[0].id, 1);
    assert_eq!(output.status.as_deref(), Some("submitted"));
    assert_eq!(output.search.as_deref(), Some("rust"));
}

#[tokio::test]
async fn denies_without_view_or_nominate_permission() {
    let mut store = FakeStore::denied();

    let error = list_organization_teacher_applications(
        &mut store,
        OrganizationTeacherApplicationListQuery {
            actor_user_id: 7,
            organization_id: 9,
            status: None,
            search: None,
            limit: None,
            offset: None,
        },
    )
    .await
    .expect_err("list should be permission gated");

    assert!(matches!(
        error,
        OrganizationTeacherApplicationListError::PermissionDenied(_)
    ));
}

struct FakeStore {
    can_view: bool,
    can_nominate: bool,
}

impl FakeStore {
    fn allowed() -> Self {
        Self {
            can_view: true,
            can_nominate: true,
        }
    }

    fn denied() -> Self {
        Self {
            can_view: false,
            can_nominate: false,
        }
    }
}

impl OrganizationTeacherApplicationListStore for FakeStore {
    fn organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<
            OrganizationTeacherApplicationOrganizationOutput,
            OrganizationTeacherApplicationListError,
        >,
    > {
        ready(Ok(OrganizationTeacherApplicationOrganizationOutput {
            id: organization_id,
            name: "Org".to_string(),
        }))
        .boxed()
    }

    fn can_view_applications(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>> {
        ready(Ok(self.can_view)).boxed()
    }

    fn can_nominate_teachers(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>> {
        ready(Ok(self.can_nominate)).boxed()
    }

    fn list_teacher_applications(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError>,
    > {
        ready(Ok(OrganizationTeacherApplicationDataset {
            applications: vec![
                application(1, "submitted", "Rust mentor"),
                application(2, "approved", "Python mentor"),
            ],
            summary: TeacherApplicationDashboardSummaryOutput {
                submitted: 1,
                approved: 1,
                total: 2,
                ..Default::default()
            },
            operator_permissions: OrganizationTeacherApplicationPermissionsOutput {
                can_view_applications: false,
                can_nominate_teachers: false,
            },
        }))
        .boxed()
    }
}

fn application(
    id: i64,
    status: &str,
    experience_summary: &str,
) -> OrganizationTeacherApplicationItemOutput {
    OrganizationTeacherApplicationItemOutput {
        id,
        applicant: TeacherApplicationUserSummaryOutput {
            id: 4,
            name: "Candidate".to_string(),
            email: "candidate@example.com".to_string(),
        },
        requested_scope: "organization".to_string(),
        requested_organization: None,
        requested_course: None,
        sponsored_by_this_organization: true,
        requested_for_this_organization: false,
        experience_summary: experience_summary.to_string(),
        portfolio_links: Vec::new(),
        status: status.to_string(),
        reviewer: None,
        decision_reason: None,
        audit: Default::default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        decided_at: None,
    }
}
