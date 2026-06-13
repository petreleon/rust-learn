use futures::future::BoxFuture;

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError,
};

pub trait TeacherApplicationPlatformReviewStore {
    fn can_review_teacher_applications(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>>;

    fn can_approve_teacher_application(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>>;

    fn can_reject_teacher_application(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>>;

    fn list_applications(
        &mut self,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError>,
    >;
}
