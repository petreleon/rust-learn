use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::update_course_lifecycle::{
    update_course_lifecycle, CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput,
    CourseLifecycleStore,
};

#[derive(Default)]
struct FakeCourseLifecycleStore {
    course_permission: bool,
    platform_permission: bool,
    requested_course_permission: Option<String>,
    updated_status: Option<String>,
}

impl AccessDecisionStore for FakeCourseLifecycleStore {
    type Error = CourseLifecycleError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, CourseLifecycleError>> {
        let allowed = match scope {
            AccessScope::Course(_) => {
                self.requested_course_permission = Some(action.permission_name().to_string());
                self.course_permission
            }
            AccessScope::Platform(_) => self.platform_permission,
            AccessScope::Organization(_) => false,
        };
        async move { Ok(allowed) }.boxed()
    }
}

impl CourseLifecycleStore for FakeCourseLifecycleStore {
    fn update_status(
        &mut self,
        course_id: i32,
        status: String,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>> {
        self.updated_status = Some(status.clone());
        async move {
            Ok(CourseLifecycleOutput {
                id: course_id,
                title: "Lifecycle course".to_string(),
                lifecycle_status: status,
                description: None,
                topics: None,
                prerequisites: None,
            })
        }
        .boxed()
    }
}

#[tokio::test]
async fn update_course_lifecycle_normalizes_status_and_checks_permission() {
    let mut store = FakeCourseLifecycleStore {
        course_permission: true,
        ..Default::default()
    };

    let result = update_course_lifecycle(
        &mut store,
        CourseLifecycleCommand {
            actor_user_id: 5,
            course_id: 17,
            status: " Published ".to_string(),
        },
    )
    .await
    .expect("lifecycle update should succeed");

    assert_eq!(result.lifecycle_status, "published");
    assert_eq!(
        store.requested_course_permission.as_deref(),
        Some("PUBLISH_CONTENT")
    );
    assert_eq!(store.updated_status.as_deref(), Some("published"));
}

#[tokio::test]
async fn update_course_lifecycle_rejects_invalid_status_before_store_update() {
    let mut store = FakeCourseLifecycleStore {
        course_permission: true,
        ..Default::default()
    };

    let error = update_course_lifecycle(
        &mut store,
        CourseLifecycleCommand {
            actor_user_id: 5,
            course_id: 17,
            status: "launched".to_string(),
        },
    )
    .await
    .expect_err("invalid status should fail");

    assert!(matches!(error, CourseLifecycleError::InvalidStatus(_)));
    assert_eq!(store.updated_status, None);
}
