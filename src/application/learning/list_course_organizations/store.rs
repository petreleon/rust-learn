use futures::future::BoxFuture;

use crate::application::learning::list_course_organizations::{
    CourseOrganizationOutput, CourseOrganizationReadError,
};

pub trait CourseOrganizationStore {
    fn list_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError>>;
}
