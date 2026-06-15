use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::learner_progress::{
    save_learner_progress, LearnerProgressError, LearnerProgressOutput, LearnerProgressStore,
    ProgressCourse, SaveLearnerProgressCommand,
};

#[derive(Default)]
struct FakeLearnerProgressStore {
    lifecycle_status: String,
    roles: Vec<String>,
    approved_join: bool,
    content_belongs: bool,
    saved_content_id: Option<i32>,
}

impl AccessDecisionStore for FakeLearnerProgressStore {
    type Error = LearnerProgressError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        assert_eq!(action.permission_name(), "VIEW_COURSE");
        assert!(matches!(
            scope,
            AccessScope::Course(_) | AccessScope::Organization(_)
        ));
        async move { Ok(false) }.boxed()
    }
}

impl LearnerProgressStore for FakeLearnerProgressStore {
    fn course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<ProgressCourse, LearnerProgressError>> {
        let lifecycle_status = self.lifecycle_status.clone();
        async move {
            Ok(ProgressCourse {
                id: course_id,
                lifecycle_status,
            })
        }
        .boxed()
    }

    fn course_organization_ids(
        &mut self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, LearnerProgressError>> {
        async move { Ok(Vec::new()) }.boxed()
    }

    fn actor_course_roles(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<String>, LearnerProgressError>> {
        let roles = self.roles.clone();
        async move { Ok(roles) }.boxed()
    }

    fn has_join_request_status(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
        _status: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        let approved_join = self.approved_join;
        async move { Ok(approved_join) }.boxed()
    }

    fn content_belongs_to_course(
        &mut self,
        _course_id: i32,
        _content_id: i32,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        let content_belongs = self.content_belongs;
        async move { Ok(content_belongs) }.boxed()
    }

    fn save_progress(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<LearnerProgressOutput, LearnerProgressError>> {
        self.saved_content_id = Some(content_id);
        async move {
            Ok(LearnerProgressOutput {
                id: 1,
                user_id: actor_user_id,
                course_id,
                content_id,
                viewed_at: Utc::now(),
            })
        }
        .boxed()
    }

    fn get_progress(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Option<LearnerProgressOutput>, LearnerProgressError>> {
        async move { Ok(None) }.boxed()
    }
}

#[tokio::test]
async fn save_progress_requires_published_enrolled_course_and_matching_content() {
    let mut store = FakeLearnerProgressStore {
        lifecycle_status: "published".to_string(),
        roles: vec!["STUDENT".to_string()],
        content_belongs: true,
        ..Default::default()
    };

    let result = save_learner_progress(
        &mut store,
        SaveLearnerProgressCommand {
            actor_user_id: 5,
            course_id: 17,
            content_id: 31,
        },
    )
    .await
    .expect("enrolled learner should save progress");

    assert_eq!(result.content_id, 31);
    assert_eq!(store.saved_content_id, Some(31));
}

#[tokio::test]
async fn save_progress_rejects_visible_course_without_enrollment() {
    let mut store = FakeLearnerProgressStore {
        lifecycle_status: "published".to_string(),
        content_belongs: true,
        ..Default::default()
    };

    let error = save_learner_progress(
        &mut store,
        SaveLearnerProgressCommand {
            actor_user_id: 5,
            course_id: 17,
            content_id: 31,
        },
    )
    .await
    .expect_err("non-enrolled learner should fail");

    assert!(matches!(error, LearnerProgressError::PermissionDenied(_)));
    assert_eq!(store.saved_content_id, None);
}
