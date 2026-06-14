use crate::application::learning::list_course_organizations::{
    CourseOrganizationOutput, CourseOrganizationReadError, CourseOrganizationStore,
};

pub async fn list_course_organizations(
    store: &mut impl CourseOrganizationStore,
    course_id: i32,
) -> Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError> {
    store.list_for_course(course_id).await
}
