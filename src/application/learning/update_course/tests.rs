use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::update_course::{
    update_course, CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput, CourseUpdatePatch,
    CourseUpdateStore,
};

#[derive(Default)]
struct FakeCourseUpdateStore {
    course_permission: bool,
    platform_permission: bool,
    requested_course_permission: Option<String>,
    updated_patch: Option<CourseUpdatePatch>,
}

impl AccessDecisionStore for FakeCourseUpdateStore {
    type Error = CourseUpdateError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, CourseUpdateError>> {
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

impl CourseUpdateStore for FakeCourseUpdateStore {
    fn update_course(
        &mut self,
        course_id: i32,
        patch: CourseUpdatePatch,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>> {
        self.updated_patch = Some(patch.clone());
        async move {
            Ok(CourseUpdateOutput {
                id: course_id,
                title: patch.title.unwrap_or_else(|| "Existing title".to_string()),
                lifecycle_status: "draft".to_string(),
                description: patch.description.flatten(),
                topics: patch.topics.flatten(),
                prerequisites: patch.prerequisites.flatten(),
            })
        }
        .boxed()
    }
}

#[tokio::test]
async fn update_course_checks_settings_permission_and_applies_patch() {
    let mut store = FakeCourseUpdateStore {
        course_permission: true,
        ..Default::default()
    };

    let result = update_course(
        &mut store,
        CourseUpdateCommand {
            actor_user_id: 5,
            course_id: 17,
            title: Some("Updated".to_string()),
            description: Some(Some("Description".to_string())),
            topics: None,
            prerequisites: Some(None),
        },
    )
    .await
    .expect("course update should succeed");

    assert_eq!(result.title, "Updated");
    assert_eq!(
        store.requested_course_permission.as_deref(),
        Some("MANAGE_COURSE_SETTINGS")
    );
    assert_eq!(
        store
            .updated_patch
            .as_ref()
            .and_then(|patch| patch.title.as_deref()),
        Some("Updated")
    );
}

#[tokio::test]
async fn update_course_rejects_actor_without_course_or_platform_permission() {
    let mut store = FakeCourseUpdateStore::default();

    let error = update_course(
        &mut store,
        CourseUpdateCommand {
            actor_user_id: 5,
            course_id: 17,
            title: Some("Denied".to_string()),
            description: None,
            topics: None,
            prerequisites: None,
        },
    )
    .await
    .expect_err("missing permission should fail");

    assert!(matches!(error, CourseUpdateError::PermissionDenied(_)));
    assert_eq!(store.updated_patch, None);
}
