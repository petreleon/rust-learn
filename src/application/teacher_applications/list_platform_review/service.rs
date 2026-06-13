use futures::future::BoxFuture;

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewError, TeacherApplicationPlatformReviewOutput,
    TeacherApplicationPlatformReviewQuery,
};

pub trait TeacherApplicationPlatformReviewUseCase: Send + Sync {
    fn list_platform_review_applications(
        &self,
        query: TeacherApplicationPlatformReviewQuery,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewOutput, TeacherApplicationPlatformReviewError>,
    >;
}
