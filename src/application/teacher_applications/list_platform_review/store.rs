use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError,
};

pub trait TeacherApplicationPlatformReviewStore:
    AccessDecisionStore<Error = TeacherApplicationPlatformReviewError>
{
    fn list_applications(
        &mut self,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError>,
    >;
}
