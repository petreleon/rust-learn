use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::create_course::{
    create_course, CourseCreationCommand, CourseCreationError, CourseCreationOutput,
    CourseCreationStore,
};

#[derive(Default)]
struct FakeCourseCreationStore {
    platform_permission: bool,
    organization_permission: bool,
    requested_organization_id: Option<i32>,
    created_title: Option<String>,
    created_organization_ids: Vec<i32>,
}

impl AccessDecisionStore for FakeCourseCreationStore {
    type Error = CourseCreationError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, CourseCreationError>> {
        let allowed = match scope {
            AccessScope::Platform(_) => self.platform_permission,
            AccessScope::Organization(scope) => {
                self.requested_organization_id = Some(scope.organization_id());
                self.organization_permission
            }
            AccessScope::Course(_) => false,
        };
        assert_eq!(action.permission_name(), "CREATE_COURSE");
        async move { Ok(allowed) }.boxed()
    }
}

impl CourseCreationStore for FakeCourseCreationStore {
    fn create_course(
        &mut self,
        title: String,
        organization_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>> {
        self.created_title = Some(title.clone());
        self.created_organization_ids = organization_ids.clone();
        async move {
            Ok(CourseCreationOutput {
                id: 21,
                title,
                lifecycle_status: "draft".to_string(),
                description: None,
                topics: None,
                prerequisites: None,
            })
        }
        .boxed()
    }
}

#[tokio::test]
async fn create_course_allows_owner_organization_permission() {
    let mut store = FakeCourseCreationStore {
        organization_permission: true,
        ..Default::default()
    };

    let result = create_course(
        &mut store,
        CourseCreationCommand {
            actor_user_id: 5,
            title: "Org Course".to_string(),
            organization_ids: vec![7, 8],
        },
    )
    .await
    .expect("organization permission should create course");

    assert_eq!(result.title, "Org Course");
    assert_eq!(store.requested_organization_id, Some(7));
    assert_eq!(store.created_organization_ids, vec![7, 8]);
}

#[tokio::test]
async fn create_course_rejects_actor_without_platform_or_owner_org_permission() {
    let mut store = FakeCourseCreationStore::default();

    let error = create_course(
        &mut store,
        CourseCreationCommand {
            actor_user_id: 5,
            title: "Denied".to_string(),
            organization_ids: vec![7],
        },
    )
    .await
    .expect_err("missing permission should fail");

    assert!(matches!(error, CourseCreationError::PermissionDenied(_)));
    assert_eq!(store.created_title, None);
}
