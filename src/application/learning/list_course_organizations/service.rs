use futures::future::BoxFuture;

use crate::application::learning::list_course_organizations::{
    CourseOrganizationOutput, CourseOrganizationReadError,
};

pub trait CourseOrganizationsUseCase: Send + Sync {
    fn list_course_organizations(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError>>;
}
