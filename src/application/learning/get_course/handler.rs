use crate::application::learning::get_course::{CourseOutput, CourseReadError, CourseReadStore};

pub async fn get_course(
    store: &mut impl CourseReadStore,
    course_id: i32,
) -> Result<CourseOutput, CourseReadError> {
    store.get(course_id).await
}
